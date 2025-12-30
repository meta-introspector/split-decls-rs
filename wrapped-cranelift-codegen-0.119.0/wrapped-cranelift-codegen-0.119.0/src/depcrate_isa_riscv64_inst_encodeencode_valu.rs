// Generated macro for encode_valu (function)
macro_rules! Depcrate_isa_riscv64_inst_encodeencode_valu {
() => {
// Module: crate::isa::riscv64::inst::encode
// Provides: {"encode_valu"}
// Dependencies: {}
# [doc = " Encodes a Vector ALU instruction."] # [doc = ""] # [doc = " Fields:"] # [doc = " - opcode (7 bits)"] # [doc = " - vd     (5 bits)"] # [doc = " - funct3 (3 bits)"] # [doc = " - vs1    (5 bits)"] # [doc = " - vs2    (5 bits)"] # [doc = " - vm     (1 bit)"] # [doc = " - funct6 (6 bits)"] # [doc = ""] # [doc = " See: https://github.com/riscv/riscv-v-spec/blob/master/valu-format.adoc"] pub fn encode_valu (op : VecAluOpRRR , vd : WritableReg , vs1 : Reg , vs2 : Reg , masking : VecOpMasking ,) -> u32 { let funct7 = (op . funct6 () << 1) | masking . encode () ; encode_r_type_bits (op . opcode () , reg_to_gpr_num (vd . to_reg ()) , op . funct3 () , reg_to_gpr_num (vs1) , reg_to_gpr_num (vs2) , funct7 ,) }
};
}
