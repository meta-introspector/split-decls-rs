// Generated macro for encode_valu_rrr_imm (function)
macro_rules! Depcrate_isa_riscv64_inst_encodeencode_valu_rrr_imm {
() => {
// Module: crate::isa::riscv64::inst::encode
// Provides: {"encode_valu_rrr_imm"}
// Dependencies: {}
pub fn encode_valu_rrr_imm (op : VecAluOpRRRImm5 , vd : WritableReg , imm : Imm5 , vs2 : Reg , masking : VecOpMasking ,) -> u32 { let funct7 = (op . funct6 () << 1) | masking . encode () ; let imm = imm . bits () as u32 ; encode_r_type_bits (op . opcode () , reg_to_gpr_num (vd . to_reg ()) , op . funct3 () , imm , reg_to_gpr_num (vs2) , funct7 ,) }
};
}
