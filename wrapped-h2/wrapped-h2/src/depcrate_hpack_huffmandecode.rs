// Generated macro for decode (function)
macro_rules! Depcrate_hpack_huffmandecode {
() => {
// Module: crate::hpack::huffman
// Provides: {"decode"}
// Dependencies: {}
pub fn decode (src : & [u8] , buf : & mut BytesMut) -> Result < BytesMut , DecoderError > { let mut decoder = Decoder :: new () ; buf . reserve (src . len () << 1) ; for b in src { if let Some (b) = decoder . decode4 (b >> 4) ? { buf . put_u8 (b) ; } if let Some (b) = decoder . decode4 (b & 0xf) ? { buf . put_u8 (b) ; } } if ! decoder . is_final () { return Err (DecoderError :: InvalidHuffmanCode) ; } Ok (buf . split ()) }
};
}
