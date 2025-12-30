// Generated macro for encode_fli (function)
macro_rules! Depcrate_isa_riscv64_inst_encodeencode_fli {
() => {
// Module: crate::isa::riscv64::inst::encode
// Provides: {"encode_fli"}
// Dependencies: {}
pub fn encode_fli (ty : Type , imm : FliConstant , rd : WritableReg) -> u32 { let op = FpuOPRR :: FmvFmtX ; let width = FpuOPWidth :: try_from (ty) . unwrap () ; let frm = 0 ; let rs2 = 1 ; let mut bits = 0 ; bits |= unsigned_field_width (op . opcode () , 7) ; bits |= reg_to_gpr_num (rd . to_reg ()) << 7 ; bits |= unsigned_field_width (frm , 3) << 12 ; bits |= unsigned_field_width (imm . bits () as u32 , 5) << 15 ; bits |= unsigned_field_width (rs2 , 6) << 20 ; bits |= unsigned_field_width (op . funct7 (width) , 7) << 25 ; bits }
};
}
