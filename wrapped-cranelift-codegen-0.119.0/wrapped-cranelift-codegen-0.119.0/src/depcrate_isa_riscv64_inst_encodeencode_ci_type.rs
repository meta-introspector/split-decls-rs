// Generated macro for encode_ci_type (function)
macro_rules! Depcrate_isa_riscv64_inst_encodeencode_ci_type {
() => {
// Module: crate::isa::riscv64::inst::encode
// Provides: {"encode_ci_type"}
// Dependencies: {}
pub fn encode_ci_type (op : CiOp , rd : WritableReg , imm : Imm6) -> u16 { let imm = imm . bits () ; let mut bits = 0 ; bits |= unsigned_field_width (op . op () . bits () , 2) ; bits |= unsigned_field_width ((imm & 0x1f) as u32 , 5) << 2 ; bits |= reg_to_gpr_num (rd . to_reg ()) << 7 ; bits |= unsigned_field_width (((imm >> 5) & 1) as u32 , 1) << 12 ; bits |= unsigned_field_width (op . funct3 () , 3) << 13 ; bits . try_into () . unwrap () }
};
}
