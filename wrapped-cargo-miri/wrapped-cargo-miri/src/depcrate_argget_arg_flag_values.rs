// Generated macro for get_arg_flag_values (function)
macro_rules! Depcrate_argget_arg_flag_values {
() => {
// Module: crate::arg
// Provides: {"get_arg_flag_values"}
// Dependencies: {}
# [doc = " Gets the values of a `--flag`."] pub fn get_arg_flag_values (name : & str) -> impl Iterator < Item = String > + '_ { ArgFlagValueIter :: from_string_iter (env :: args () , name) }
};
}
