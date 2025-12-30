// Generated macro for encode_csr_reg (function)
macro_rules! Depcrate_isa_riscv64_inst_encodeencode_csr_reg {
() => {
// Module: crate::isa::riscv64::inst::encode
// Provides: {"encode_csr_reg"}
// Dependencies: {}
pub fn encode_csr_reg (op : CsrRegOP , rd : WritableReg , rs : Reg , csr : CSR) -> u32 { encode_i_type (op . opcode () , rd , op . funct3 () , rs , csr . bits ()) }
};
}
