// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/panic.rs
// Error: expected square brackets
// Problematic line: line 9

mod panic_info;
mod unwind_safe;

#[stable(feature = "panic_hooks", since = "1.10.0")]
pub use self::location::Location;
#[stable(feature = "panic_hooks", since = "1.10.0")]
pub use self::panic_info::PanicInfo;
