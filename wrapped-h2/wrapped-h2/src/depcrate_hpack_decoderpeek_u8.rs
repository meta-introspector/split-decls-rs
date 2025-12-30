// Generated macro for peek_u8 (function)
macro_rules! Depcrate_hpack_decoderpeek_u8 {
() => {
// Module: crate::hpack::decoder
// Provides: {"peek_u8"}
// Dependencies: {}
fn peek_u8 < B : Buf > (buf : & B) -> Option < u8 > { if buf . has_remaining () { Some (buf . chunk () [0]) } else { None } }
};
}
