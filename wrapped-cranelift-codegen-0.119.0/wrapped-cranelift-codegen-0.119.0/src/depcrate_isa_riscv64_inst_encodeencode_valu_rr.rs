// Generated macro for encode_valu_rr (function)
macro_rules! Depcrate_isa_riscv64_inst_encodeencode_valu_rr {
() => {
// Module: crate::isa::riscv64::inst::encode
// Provides: {"encode_valu_rr"}
// Dependencies: {}
pub fn encode_valu_rr (op : VecAluOpRR , vd : WritableReg , vs : Reg , masking : VecOpMasking) -> u32 { let funct7 = (op . funct6 () << 1) | masking . encode () ; let (vs1 , vs2) = if op . vs_is_vs2_encoded () { (op . aux_encoding () , reg_to_gpr_num (vs)) } else { (reg_to_gpr_num (vs) , op . aux_encoding ()) } ; encode_r_type_bits (op . opcode () , reg_to_gpr_num (vd . to_reg ()) , op . funct3 () , vs1 , vs2 , funct7 ,) }
};
}
