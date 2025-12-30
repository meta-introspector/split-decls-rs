// Generated macro for encode_sib (function)
macro_rules! Depcrate_isa_x64_encoding_rexencode_sib {
() => {
// Module: crate::isa::x64::encoding::rex
// Provides: {"encode_sib"}
// Dependencies: {}
# [inline (always)] pub (crate) fn encode_sib (shift : u8 , enc_index : u8 , enc_base : u8) -> u8 { debug_assert ! (shift < 4) ; debug_assert ! (enc_index < 8) ; debug_assert ! (enc_base < 8) ; ((shift & 3) << 6) | ((enc_index & 7) << 3) | (enc_base & 7) }
};
}
