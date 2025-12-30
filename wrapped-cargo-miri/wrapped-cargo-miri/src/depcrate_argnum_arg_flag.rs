// Generated macro for num_arg_flag (function)
macro_rules! Depcrate_argnum_arg_flag {
() => {
// Module: crate::arg
// Provides: {"num_arg_flag"}
// Dependencies: {}
# [doc = " Determines how many times a `--flag` is present."] pub fn num_arg_flag (name : & str) -> usize { env :: args () . take_while (| val | val != "--") . filter (| val | val == name) . count () }
};
}
