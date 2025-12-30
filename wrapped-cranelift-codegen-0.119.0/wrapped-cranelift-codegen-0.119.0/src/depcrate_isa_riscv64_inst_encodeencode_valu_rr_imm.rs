// Generated macro for encode_valu_rr_imm (function)
macro_rules! Depcrate_isa_riscv64_inst_encodeencode_valu_rr_imm {
() => {
// Module: crate::isa::riscv64::inst::encode
// Provides: {"encode_valu_rr_imm"}
// Dependencies: {}
# [doc = " Encodes a Vector ALU+Imm instruction."] # [doc = " This is just a Vector ALU instruction with an immediate in the VS1 field."] # [doc = ""] # [doc = " Fields:"] # [doc = " - opcode (7 bits)"] # [doc = " - vd     (5 bits)"] # [doc = " - funct3 (3 bits)"] # [doc = " - imm    (5 bits)"] # [doc = " - vs2    (5 bits)"] # [doc = " - vm     (1 bit)"] # [doc = " - funct6 (6 bits)"] # [doc = ""] # [doc = " See: https://github.com/riscv/riscv-v-spec/blob/master/valu-format.adoc"] pub fn encode_valu_rr_imm (op : VecAluOpRRImm5 , vd : WritableReg , imm : Imm5 , vs2 : Reg , masking : VecOpMasking ,) -> u32 { let funct7 = (op . funct6 () << 1) | masking . encode () ; let imm = imm . bits () as u32 ; encode_r_type_bits (op . opcode () , reg_to_gpr_num (vd . to_reg ()) , op . funct3 () , imm , reg_to_gpr_num (vs2) , funct7 ,) }
};
}
