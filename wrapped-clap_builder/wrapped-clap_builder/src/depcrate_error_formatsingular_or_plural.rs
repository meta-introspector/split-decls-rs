// Generated macro for singular_or_plural (function)
macro_rules! Depcrate_error_formatsingular_or_plural {
() => {
// Module: crate::error::format
// Provides: {"singular_or_plural"}
// Dependencies: {}
# [doc = " Returns the singular or plural form on the verb to be based on the argument's value."] fn singular_or_plural (n : usize) -> & 'static str { if n > 1 { " were provided" } else { " was provided" } }
};
}
