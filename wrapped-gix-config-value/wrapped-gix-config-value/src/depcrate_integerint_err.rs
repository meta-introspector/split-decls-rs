// Generated macro for int_err (function)
macro_rules! Depcrate_integerint_err {
() => {
// Module: crate::integer
// Provides: {"int_err"}
// Dependencies: {}
fn int_err (input : impl Into < BString >) -> Error { Error :: new ("Integers needs to be positive or negative numbers which may have a suffix like 1k, 42, or 50G" , input ,) }
};
}
