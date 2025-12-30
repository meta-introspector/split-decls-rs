// Generated macro for encode_cr_type (function)
macro_rules! Depcrate_isa_riscv64_inst_encodeencode_cr_type {
() => {
// Module: crate::isa::riscv64::inst::encode
// Provides: {"encode_cr_type"}
// Dependencies: {}
pub fn encode_cr_type (op : CrOp , rd : WritableReg , rs2 : Reg) -> u16 { let mut bits = 0 ; bits |= unsigned_field_width (op . op () . bits () , 2) ; bits |= reg_to_gpr_num (rs2) << 2 ; bits |= reg_to_gpr_num (rd . to_reg ()) << 7 ; bits |= unsigned_field_width (op . funct4 () , 4) << 12 ; bits . try_into () . unwrap () }
};
}
