// Generated macro for encode_csr_imm (function)
macro_rules! Depcrate_isa_riscv64_inst_encodeencode_csr_imm {
() => {
// Module: crate::isa::riscv64::inst::encode
// Provides: {"encode_csr_imm"}
// Dependencies: {}
pub fn encode_csr_imm (op : CsrImmOP , rd : WritableReg , csr : CSR , imm : UImm5) -> u32 { encode_i_type_bits (op . opcode () , reg_to_gpr_num (rd . to_reg ()) , op . funct3 () , imm . bits () , csr . bits () . bits () ,) }
};
}
