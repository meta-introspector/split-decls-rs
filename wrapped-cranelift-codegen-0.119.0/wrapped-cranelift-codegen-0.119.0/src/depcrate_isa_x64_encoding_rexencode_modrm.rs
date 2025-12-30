// Generated macro for encode_modrm (function)
macro_rules! Depcrate_isa_x64_encoding_rexencode_modrm {
() => {
// Module: crate::isa::x64::encoding::rex
// Provides: {"encode_modrm"}
// Dependencies: {}
# [doc = " Encode the ModR/M byte."] # [inline (always)] pub fn encode_modrm (m0d : u8 , enc_reg_g : u8 , rm_e : u8) -> u8 { debug_assert ! (m0d < 4) ; debug_assert ! (enc_reg_g < 8) ; debug_assert ! (rm_e < 8) ; ((m0d & 3) << 6) | ((enc_reg_g & 7) << 3) | (rm_e & 7) }
};
}
