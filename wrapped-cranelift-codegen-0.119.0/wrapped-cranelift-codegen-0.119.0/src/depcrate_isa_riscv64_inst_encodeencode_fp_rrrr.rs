// Generated macro for encode_fp_rrrr (function)
macro_rules! Depcrate_isa_riscv64_inst_encodeencode_fp_rrrr {
() => {
// Module: crate::isa::riscv64::inst::encode
// Provides: {"encode_fp_rrrr"}
// Dependencies: {}
pub fn encode_fp_rrrr (op : FpuOPRRRR , width : FpuOPWidth , frm : FRM , rd : WritableReg , rs1 : Reg , rs2 : Reg , rs3 : Reg ,) -> u32 { let funct7 = (reg_to_gpr_num (rs3) << 2) | width . as_u32 () ; encode_r_type_bits (op . opcode () , reg_to_gpr_num (rd . to_reg ()) , frm . as_u32 () , reg_to_gpr_num (rs1) , reg_to_gpr_num (rs2) , funct7 ,) }
};
}
