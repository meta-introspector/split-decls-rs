// Generated macro for encode_cj_type (function)
macro_rules! Depcrate_isa_riscv64_inst_encodeencode_cj_type {
() => {
// Module: crate::isa::riscv64::inst::encode
// Provides: {"encode_cj_type"}
// Dependencies: {}
pub fn encode_cj_type (op : CjOp , imm : Imm12) -> u16 { let imm = imm . bits () ; debug_assert ! (imm & 1 == 0) ; let mut imm_field = 0 ; imm_field |= ((imm >> 11) & 1) << 10 ; imm_field |= ((imm >> 4) & 1) << 9 ; imm_field |= ((imm >> 8) & 3) << 7 ; imm_field |= ((imm >> 10) & 1) << 6 ; imm_field |= ((imm >> 6) & 1) << 5 ; imm_field |= ((imm >> 7) & 1) << 4 ; imm_field |= ((imm >> 1) & 7) << 1 ; imm_field |= ((imm >> 5) & 1) << 0 ; let mut bits = 0 ; bits |= unsigned_field_width (op . op () . bits () , 2) ; bits |= unsigned_field_width (imm_field , 11) << 2 ; bits |= unsigned_field_width (op . funct3 () , 3) << 13 ; bits . try_into () . unwrap () }
};
}
