// Generated macro for arg_value (function)
macro_rules! Depcrate_testarg_value {
() => {
// Module: crate::test
// Provides: {"arg_value"}
// Dependencies: {}
pub (crate) fn arg_value (name : impl AsRef < str > , value : impl AsRef < str >) -> ArgumentValue { ArgumentValue :: new (pat (name) , expr (value)) }
};
}
