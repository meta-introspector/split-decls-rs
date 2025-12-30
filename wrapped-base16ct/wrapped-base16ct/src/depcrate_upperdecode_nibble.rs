// Generated macro for decode_nibble (function)
macro_rules! Depcrate_upperdecode_nibble {
() => {
// Module: crate::upper
// Provides: {"decode_nibble"}
// Dependencies: {}
# [doc = " Decode a single nibble of upper hex"] # [inline (always)] fn decode_nibble (src : u8) -> u16 { let byte = src as i16 ; let mut ret : i16 = - 1 ; ret += (((0x2fi16 - byte) & (byte - 0x3a)) >> 8) & (byte - 47) ; ret += (((0x40i16 - byte) & (byte - 0x47)) >> 8) & (byte - 54) ; ret as u16 }
};
}
