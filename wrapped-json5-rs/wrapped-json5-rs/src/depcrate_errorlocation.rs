// Generated macro for Location (struct)
macro_rules! Depcrate_errorLocation {
() => {
// Module: crate::error
// Provides: {"Location"}
// Dependencies: {}
# [doc = " One-based line and column at which the error was detected."] # [derive (Clone , Debug , PartialEq)] pub struct Location { # [doc = " The one-based line number of the error."] pub line : usize , # [doc = " The one-based column number of the error."] pub column : usize , }
};
}
