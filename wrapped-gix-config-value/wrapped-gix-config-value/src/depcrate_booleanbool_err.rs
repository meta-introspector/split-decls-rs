// Generated macro for bool_err (function)
macro_rules! Depcrate_booleanbool_err {
() => {
// Module: crate::boolean
// Provides: {"bool_err"}
// Dependencies: {}
fn bool_err (input : impl Into < BString >) -> Error { Error :: new ("Booleans need to be 'no', 'off', 'false', '' or 'yes', 'on', 'true' or any number" , input ,) }
};
}
