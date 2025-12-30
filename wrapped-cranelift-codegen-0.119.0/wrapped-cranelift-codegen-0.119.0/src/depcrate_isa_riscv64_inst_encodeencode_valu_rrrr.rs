// Generated macro for encode_valu_rrrr (function)
macro_rules! Depcrate_isa_riscv64_inst_encodeencode_valu_rrrr {
() => {
// Module: crate::isa::riscv64::inst::encode
// Provides: {"encode_valu_rrrr"}
// Dependencies: {}
pub fn encode_valu_rrrr (op : VecAluOpRRRR , vd : WritableReg , vs2 : Reg , vs1 : Reg , masking : VecOpMasking ,) -> u32 { let funct7 = (op . funct6 () << 1) | masking . encode () ; encode_r_type_bits (op . opcode () , reg_to_gpr_num (vd . to_reg ()) , op . funct3 () , reg_to_gpr_num (vs1) , reg_to_gpr_num (vs2) , funct7 ,) }
};
}
