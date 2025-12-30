// Generated macro for encode_i_type (function)
macro_rules! Depcrate_isa_riscv64_inst_encodeencode_i_type {
() => {
// Module: crate::isa::riscv64::inst::encode
// Provides: {"encode_i_type"}
// Dependencies: {}
# [doc = " Encode an I-type instruction."] pub fn encode_i_type (opcode : u32 , rd : WritableReg , width : u32 , rs1 : Reg , offset : Imm12) -> u32 { encode_i_type_bits (opcode , reg_to_gpr_num (rd . to_reg ()) , width , reg_to_gpr_num (rs1) , offset . bits () ,) }
};
}
