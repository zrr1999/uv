use std::sync::atomic::AtomicBool;
use std::sync::Mutex;

// macro hygiene: The user might not have direct dependencies on those crates
#[doc(hidden)]
pub use colored;
use once_cell::sync::Lazy;
use rustc_hash::FxHashSet;

/// Whether user-facing warnings are enabled.
pub static ENABLED: AtomicBool = AtomicBool::new(false);

/// Enable user-facing warnings.
pub fn enable() {
    ENABLED.store(true, std::sync::atomic::Ordering::SeqCst);
}

/// Warn a user, if warnings are enabled.
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        use $crate::colored::Colorize;

        if $crate::ENABLED.load(std::sync::atomic::Ordering::SeqCst) {
            let message = format!("{}", format_args!($($arg)*));
            let formatted = message.bold();
            eprintln!("{}{} {formatted}", "warning".yellow().bold(), ":".bold());
        }
    };
}

pub static WARNINGS: Lazy<Mutex<FxHashSet<String>>> = Lazy::new(Mutex::default);

/// Warn a user once, if warnings are enabled, with uniqueness determined by the content of the
/// message.
#[macro_export]
macro_rules! warn_once {
    ($($arg:tt)*) => {
        use $crate::colored::Colorize;

        if $crate::ENABLED.load(std::sync::atomic::Ordering::SeqCst) {
            if let Ok(mut states) = $crate::WARNINGS.lock() {
                let message = format!("{}", format_args!($($arg)*));
                let formatted = message.bold();
                if states.insert(message) {
                    eprintln!("{}{} {formatted}", "warning".yellow().bold(), ":".bold());
                }
            }
        }
    };
}