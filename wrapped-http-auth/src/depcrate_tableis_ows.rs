// Generated macro for is_ows (function)
macro_rules! Depcrate_tableis_ows {
() => {
// Module: crate::table
// Provides: {"is_ows"}
// Dependencies: {}
# [doc = " Returns true if the byte is valid optional whitespace as in [RFC 7230 section"] # [doc = " 3.2.3](https://datatracker.ietf.org/doc/html/rfc7230#section-3.2.3)."] # [doc = ""] # [doc = " ```text"] # [doc = "      OWS            = *( SP / HTAB )"] # [doc = "                     ; optional whitespace"] # [doc = " ```"] const fn is_ows (b : u8) -> bool { matches ! (b , b' ' | b'\t') }
};
}
