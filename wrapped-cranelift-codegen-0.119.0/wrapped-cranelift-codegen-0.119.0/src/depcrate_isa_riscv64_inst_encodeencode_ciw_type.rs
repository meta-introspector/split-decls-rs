// Generated macro for encode_ciw_type (function)
macro_rules! Depcrate_isa_riscv64_inst_encodeencode_ciw_type {
() => {
// Module: crate::isa::riscv64::inst::encode
// Provides: {"encode_ciw_type"}
// Dependencies: {}
pub fn encode_ciw_type (op : CiwOp , rd : WritableReg , imm : u8) -> u16 { let mut imm_field = 0 ; imm_field |= ((imm >> 1) & 1) << 0 ; imm_field |= ((imm >> 0) & 1) << 1 ; imm_field |= ((imm >> 4) & 15) << 2 ; imm_field |= ((imm >> 2) & 3) << 6 ; let mut bits = 0 ; bits |= unsigned_field_width (op . op () . bits () , 2) ; bits |= reg_to_compressed_gpr_num (rd . to_reg ()) << 2 ; bits |= unsigned_field_width (imm_field as u32 , 8) << 5 ; bits |= unsigned_field_width (op . funct3 () , 3) << 13 ; bits . try_into () . unwrap () }
};
}
