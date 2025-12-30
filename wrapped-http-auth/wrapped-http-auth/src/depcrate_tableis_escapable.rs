// Generated macro for is_escapable (function)
macro_rules! Depcrate_tableis_escapable {
() => {
// Module: crate::table
// Provides: {"is_escapable"}
// Dependencies: {}
# [doc = " Returns true if the byte is a valid end of a `quoted-pair`, as defined in"] # [doc = " [RFC 7230 section 3.2.6](https://datatracker.ietf.org/doc/html/rfc7230#section-3.2.6)."] const fn is_escapable (b : u8) -> bool { matches ! (b , b'\t' | b' ' | 0x21 ..= 0x7E | 0x80 ..= 0xFF) }
};
}
