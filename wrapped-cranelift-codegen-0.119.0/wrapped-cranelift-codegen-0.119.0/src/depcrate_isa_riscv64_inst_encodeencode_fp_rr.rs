// Generated macro for encode_fp_rr (function)
macro_rules! Depcrate_isa_riscv64_inst_encodeencode_fp_rr {
() => {
// Module: crate::isa::riscv64::inst::encode
// Provides: {"encode_fp_rr"}
// Dependencies: {}
pub fn encode_fp_rr (op : FpuOPRR , width : FpuOPWidth , frm : FRM , rd : WritableReg , rs : Reg) -> u32 { encode_r_type_bits (op . opcode () , reg_to_gpr_num (rd . to_reg ()) , frm . as_u32 () , reg_to_gpr_num (rs) , op . rs2 () , op . funct7 (width) ,) }
};
}
