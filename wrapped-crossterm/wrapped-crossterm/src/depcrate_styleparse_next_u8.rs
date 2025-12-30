// Generated macro for parse_next_u8 (function)
macro_rules! Depcrate_styleparse_next_u8 {
() => {
// Module: crate::style
// Provides: {"parse_next_u8"}
// Dependencies: {}
# [doc = " Utility function for ANSI parsing in Color and Colored."] # [doc = " Gets the next element of `iter` and tries to parse it as a `u8`."] fn parse_next_u8 < 'a > (iter : & mut impl Iterator < Item = & 'a str >) -> Option < u8 > { iter . next () . and_then (| s | s . parse () . ok ()) }
};
}
