// Generated macro for parse_fails_on_trailing_newline (function)
macro_rules! Depcrate_methods_read_line_without_trimparse_fails_on_trailing_newline {
() => {
// Module: crate::methods::read_line_without_trim
// Provides: {"parse_fails_on_trailing_newline"}
// Dependencies: {}
# [doc = " Will a `.parse::<ty>()` call fail if the input has a trailing newline?"] fn parse_fails_on_trailing_newline (ty : Ty < '_ >) -> bool { matches ! (ty . kind () , ty :: Float (_) | ty :: Bool | ty :: Int (_) | ty :: Uint (_)) }
};
}
