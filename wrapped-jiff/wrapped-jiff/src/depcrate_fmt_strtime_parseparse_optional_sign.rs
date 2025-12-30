// Generated macro for parse_optional_sign (function)
macro_rules! Depcrate_fmt_strtime_parseparse_optional_sign {
() => {
// Module: crate::fmt::strtime::parse
// Provides: {"parse_optional_sign"}
// Dependencies: {}
# [doc = " Parses an optional sign from the beginning of the input. If one isn't"] # [doc = " found, then the sign returned is positive."] # [doc = ""] # [doc = " This also returns the remaining unparsed input."] # [cfg_attr (feature = "perf-inline" , inline (always))] fn parse_optional_sign < 'i > (input : & 'i [u8]) -> (i64 , & 'i [u8]) { if input . is_empty () { (1 , input) } else if input [0] == b'-' { (- 1 , & input [1 ..]) } else if input [0] == b'+' { (1 , & input [1 ..]) } else { (1 , input) } }
};
}
