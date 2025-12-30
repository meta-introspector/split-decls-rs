// Generated macro for encode_fp_rrr (function)
macro_rules! Depcrate_isa_riscv64_inst_encodeencode_fp_rrr {
() => {
// Module: crate::isa::riscv64::inst::encode
// Provides: {"encode_fp_rrr"}
// Dependencies: {}
pub fn encode_fp_rrr (op : FpuOPRRR , width : FpuOPWidth , frm : FRM , rd : WritableReg , rs1 : Reg , rs2 : Reg ,) -> u32 { encode_r_type_bits (op . opcode () , reg_to_gpr_num (rd . to_reg ()) , frm . as_u32 () , reg_to_gpr_num (rs1) , reg_to_gpr_num (rs2) , op . funct7 (width) ,) }
};
}
