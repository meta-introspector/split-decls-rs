// Generated macro for Vary (struct)
macro_rules! Depcrate_common_varyVary {
() => {
// Module: crate::common::vary
// Provides: {"Vary"}
// Dependencies: {}
# [doc = " `Vary` header, defined in [RFC7231](https://tools.ietf.org/html/rfc7231#section-7.1.4)"] # [doc = ""] # [doc = " The \"Vary\" header field in a response describes what parts of a"] # [doc = " request message, aside from the method, Host header field, and"] # [doc = " request target, might influence the origin server's process for"] # [doc = " selecting and representing this response.  The value consists of"] # [doc = " either a single asterisk (\"*\") or a list of header field names"] # [doc = " (case-insensitive)."] # [doc = ""] # [doc = " # ABNF"] # [doc = ""] # [doc = " ```text"] # [doc = " Vary = \"*\" / 1#field-name"] # [doc = " ```"] # [doc = ""] # [doc = " # Example values"] # [doc = ""] # [doc = " * `accept-encoding, accept-language`"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use headers::Vary;"] # [doc = ""] # [doc = " let vary = Vary::any();"] # [doc = " ```"] # [derive (Debug , Clone , PartialEq)] pub struct Vary (FlatCsv) ;
};
}
