// Generated macro for Mode (enum)
macro_rules! Depcrate_typesMode {
() => {
// Module: crate::types
// Provides: {"Mode"}
// Dependencies: {}
# [doc = " The way the user is prompted."] # [derive (Default , Debug , Copy , Clone , Eq , PartialEq)] pub enum Mode { # [doc = " Visibly show user input."] Visible , # [doc = " Do not show user input, suitable for sensitive data."] # [default] Hidden , # [doc = " Do not prompt the user at all but rather abort with an error. This is useful in conjunction with [Options::askpass]."] Disable , }
};
}
