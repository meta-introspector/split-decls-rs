// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/num/shells/legacy_int_modules.rs
// Error: expected square brackets
// Problematic line: line 3

#![doc(hidden)]

macro_rules! legacy_int_module {
    ($T:ident) => (legacy_int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
    ($T:ident, #[$attr:meta]) => (
        #[$attr]
