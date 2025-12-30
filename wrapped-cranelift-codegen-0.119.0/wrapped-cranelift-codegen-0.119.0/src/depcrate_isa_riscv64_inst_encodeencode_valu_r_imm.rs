// Generated macro for encode_valu_r_imm (function)
macro_rules! Depcrate_isa_riscv64_inst_encodeencode_valu_r_imm {
() => {
// Module: crate::isa::riscv64::inst::encode
// Provides: {"encode_valu_r_imm"}
// Dependencies: {}
pub fn encode_valu_r_imm (op : VecAluOpRImm5 , vd : WritableReg , imm : Imm5 , masking : VecOpMasking ,) -> u32 { let funct7 = (op . funct6 () << 1) | masking . encode () ; debug_assert_eq ! (op , VecAluOpRImm5 :: VmvVI) ; let vs1 = imm . bits () as u32 ; let vs2 = op . aux_encoding () ; encode_r_type_bits (op . opcode () , reg_to_gpr_num (vd . to_reg ()) , op . funct3 () , vs1 , vs2 , funct7 ,) }
};
}
