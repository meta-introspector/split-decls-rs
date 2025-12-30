// Generated macro for I32CastError (enum)
macro_rules! Depcrate_helpersI32CastError {
() => {
// Module: crate::helpers
// Provides: {"I32CastError"}
// Dependencies: {}
# [doc = " Error returned when casting from an i32"] # [derive (Copy , Clone , Debug , displaydoc :: Display)] # [allow (clippy :: exhaustive_enums)] pub enum I32CastError { # [doc = " Less than i32::MIN"] BelowMin , # [doc = " Greater than i32::MAX"] AboveMax , }
};
}
