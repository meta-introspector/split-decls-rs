// Generated macro for encode_cr2_type (function)
macro_rules! Depcrate_isa_riscv64_inst_encodeencode_cr2_type {
() => {
// Module: crate::isa::riscv64::inst::encode
// Provides: {"encode_cr2_type"}
// Dependencies: {}
pub fn encode_cr2_type (op : CrOp , rs1 : Reg) -> u16 { encode_cr_type (op , WritableReg :: from_reg (rs1) , zero_reg ()) }
};
}
