// Generated macro for decode_nibble (function)
macro_rules! Depcrate_mixeddecode_nibble {
() => {
// Module: crate::mixed
// Provides: {"decode_nibble"}
// Dependencies: {}
# [doc = " Decode a single nibble of lower hex"] # [inline (always)] fn decode_nibble (src : u8) -> u16 { let byte = src as i16 ; let mut ret : i16 = - 1 ; ret += (((0x2fi16 - byte) & (byte - 0x3a)) >> 8) & (byte - 47) ; ret += (((0x40i16 - byte) & (byte - 0x47)) >> 8) & (byte - 54) ; ret += (((0x60i16 - byte) & (byte - 0x67)) >> 8) & (byte - 86) ; ret as u16 }
};
}
