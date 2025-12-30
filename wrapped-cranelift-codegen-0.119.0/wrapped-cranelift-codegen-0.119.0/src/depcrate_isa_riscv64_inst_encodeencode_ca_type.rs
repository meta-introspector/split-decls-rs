// Generated macro for encode_ca_type (function)
macro_rules! Depcrate_isa_riscv64_inst_encodeencode_ca_type {
() => {
// Module: crate::isa::riscv64::inst::encode
// Provides: {"encode_ca_type"}
// Dependencies: {}
pub fn encode_ca_type (op : CaOp , rd : WritableReg , rs2 : Reg) -> u16 { let mut bits = 0 ; bits |= unsigned_field_width (op . op () . bits () , 2) ; bits |= reg_to_compressed_gpr_num (rs2) << 2 ; bits |= unsigned_field_width (op . funct2 () , 2) << 5 ; bits |= reg_to_compressed_gpr_num (rd . to_reg ()) << 7 ; bits |= unsigned_field_width (op . funct6 () , 6) << 10 ; bits . try_into () . unwrap () }
};
}
