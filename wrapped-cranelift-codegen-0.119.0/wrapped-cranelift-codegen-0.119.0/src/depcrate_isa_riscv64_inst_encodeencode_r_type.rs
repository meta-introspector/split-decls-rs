// Generated macro for encode_r_type (function)
macro_rules! Depcrate_isa_riscv64_inst_encodeencode_r_type {
() => {
// Module: crate::isa::riscv64::inst::encode
// Provides: {"encode_r_type"}
// Dependencies: {}
# [doc = " Encode an R-type instruction."] pub fn encode_r_type (opcode : u32 , rd : WritableReg , funct3 : u32 , rs1 : Reg , rs2 : Reg , funct7 : u32 ,) -> u32 { encode_r_type_bits (opcode , reg_to_gpr_num (rd . to_reg ()) , funct3 , reg_to_gpr_num (rs1) , reg_to_gpr_num (rs2) , funct7 ,) }
};
}
