// Generated macro for get_arg_flag_value (function)
macro_rules! Depcrate_argget_arg_flag_value {
() => {
// Module: crate::arg
// Provides: {"get_arg_flag_value"}
// Dependencies: {}
# [doc = " Gets the value of a `--flag`."] pub fn get_arg_flag_value (name : & str) -> Option < String > { get_arg_flag_values (name) . next () }
};
}
