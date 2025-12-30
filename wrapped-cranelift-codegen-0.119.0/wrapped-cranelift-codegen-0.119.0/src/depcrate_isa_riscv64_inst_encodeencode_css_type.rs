// Generated macro for encode_css_type (function)
macro_rules! Depcrate_isa_riscv64_inst_encodeencode_css_type {
() => {
// Module: crate::isa::riscv64::inst::encode
// Provides: {"encode_css_type"}
// Dependencies: {}
pub fn encode_css_type (op : CssOp , src : Reg , imm : Uimm6) -> u16 { let imm = imm . bits () ; let low_bits = match op { CssOp :: CSwsp => 4 , CssOp :: CSdsp | CssOp :: CFsdsp => 3 , } ; let high_bits = 6 - low_bits ; let mut enc_imm = 0 ; enc_imm |= (imm & ((1 << low_bits) - 1)) << high_bits ; enc_imm |= imm >> low_bits ; let mut bits = 0 ; bits |= unsigned_field_width (op . op () . bits () , 2) ; bits |= reg_to_gpr_num (src) << 2 ; bits |= unsigned_field_width (enc_imm as u32 , 6) << 7 ; bits |= unsigned_field_width (op . funct3 () , 3) << 13 ; bits . try_into () . unwrap () }
};
}
