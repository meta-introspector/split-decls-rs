// Generated macro for encode_nibble (function)
macro_rules! Depcrate_upperencode_nibble {
() => {
// Module: crate::upper
// Provides: {"encode_nibble"}
// Dependencies: {}
# [doc = " Encode a single nibble of hex"] # [inline (always)] fn encode_nibble (src : u8) -> u8 { let mut ret = src as i16 + 0x30 ; ret += ((0x39i16 - ret) >> 8) & (0x41i16 - 0x3a) ; ret as u8 }
};
}
