// Generated macro for parse_iana_component (function)
macro_rules! Depcrate_fmt_strtime_parseparse_iana_component {
() => {
// Module: crate::fmt::strtime::parse
// Provides: {"parse_iana_component"}
// Dependencies: {}
# [doc = " Parses a single IANA name component. That is, the thing that leads all IANA"] # [doc = " time zone identifiers and the thing that must always come after a `/`. This"] # [doc = " returns an error if no component could be found."] # [cfg_attr (feature = "perf-inline" , inline (always))] fn parse_iana_component < 'i > (mut input : & 'i [u8] ,) -> Result < (& 'i [u8] , & 'i [u8]) , Error > { let mkname = parse :: slicer (input) ; if input . is_empty () { return Err (err ! ("expected the start of an IANA time zone identifier \
             name or component, but found end of input instead" ,)) ; } if ! matches ! (input [0] , b'_' | b'.' | b'A' ..= b'Z' | b'a' ..= b'z') { return Err (err ! ("expected the start of an IANA time zone identifier \
             name or component, but found {:?} instead" , escape :: Byte (input [0]) ,)) ; } input = & input [1 ..] ; let is_iana_char = | byte | { matches ! (byte , b'_' | b'.' | b'+' | b'-' | b'0' ..= b'9' | b'A' ..= b'Z' | b'a' ..= b'z' ,) } ; while ! input . is_empty () && is_iana_char (input [0]) { input = & input [1 ..] ; } Ok ((mkname (input) , input)) }
};
}
