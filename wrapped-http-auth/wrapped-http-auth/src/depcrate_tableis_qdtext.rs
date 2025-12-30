// Generated macro for is_qdtext (function)
macro_rules! Depcrate_tableis_qdtext {
() => {
// Module: crate::table
// Provides: {"is_qdtext"}
// Dependencies: {}
# [doc = " Returns true if the byte is a valid `qdtext` (excluding `obs-text`), as defined in"] # [doc = " [RFC 7230 section 3.2.6](https://datatracker.ietf.org/doc/html/rfc7230#section-3.2.6)."] # [doc = ""] # [doc = " ```text"] # [doc = " quoted-string  = DQUOTE *( qdtext / quoted-pair ) DQUOTE"] # [doc = " qdtext         = HTAB / SP /%x21 / %x23-5B / %x5D-7E / obs-text"] # [doc = " obs-text       = %x80-FF"] # [doc = " quoted-pair    = \"\\\" ( HTAB / SP / VCHAR / obs-text )"] # [doc = " VCHAR          =  %x21-7E"] # [doc = "                ; visible (printing) characters"] # [doc = " ```"] const fn is_qdtext (b : u8) -> bool { matches ! (b , b'\t' | b' ' | 0x21 | 0x23 ..= 0x5B | 0x5D ..= 0x7E) }
};
}
