// Generated macro for parse_iana (function)
macro_rules! Depcrate_fmt_strtime_parseparse_iana {
() => {
// Module: crate::fmt::strtime::parse
// Provides: {"parse_iana"}
// Dependencies: {}
# [cfg_attr (feature = "perf-inline" , inline (always))] fn parse_iana < 'i > (input : & 'i [u8]) -> Result < (& 'i str , & 'i [u8]) , Error > { let mkiana = parse :: slicer (input) ; let (_ , mut input) = parse_iana_component (input) ? ; while input . starts_with (b"/") { input = & input [1 ..] ; let (_ , unconsumed) = parse_iana_component (input) ? ; input = unconsumed ; } let iana = core :: str :: from_utf8 (mkiana (input)) . expect ("ASCII") ; Ok ((iana , input)) }
};
}
