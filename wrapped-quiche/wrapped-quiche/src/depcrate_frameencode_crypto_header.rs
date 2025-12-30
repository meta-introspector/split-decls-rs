// Generated macro for encode_crypto_header (function)
macro_rules! Depcrate_frameencode_crypto_header {
() => {
// Module: crate::frame
// Provides: {"encode_crypto_header"}
// Dependencies: {}
pub fn encode_crypto_header (offset : u64 , length : u64 , b : & mut octets :: OctetsMut ,) -> Result < () > { b . put_varint (0x06) ? ; b . put_varint (offset) ? ; b . put_varint_with_len (length , 2) ? ; Ok (()) }
};
}
