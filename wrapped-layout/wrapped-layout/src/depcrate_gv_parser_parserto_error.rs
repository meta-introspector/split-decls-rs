// Generated macro for to_error (function)
macro_rules! Depcrate_gv_parser_parserto_error {
() => {
// Module: crate::gv::parser::parser
// Provides: {"to_error"}
// Dependencies: {}
# [doc = " Creates an error from the string \\p str."] fn to_error < T > (str : & str) -> Result < T , String > { Result :: Err (str . to_string ()) }
};
}
