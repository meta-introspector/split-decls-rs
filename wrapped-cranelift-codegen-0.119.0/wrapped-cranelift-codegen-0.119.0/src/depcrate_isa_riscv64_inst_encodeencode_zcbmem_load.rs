// Generated macro for encode_zcbmem_load (function)
macro_rules! Depcrate_isa_riscv64_inst_encodeencode_zcbmem_load {
() => {
// Module: crate::isa::riscv64::inst::encode
// Provides: {"encode_zcbmem_load"}
// Dependencies: {}
pub fn encode_zcbmem_load (op : ZcbMemOp , rd : WritableReg , base : Reg , imm : Uimm2) -> u16 { encode_zcbmem_bits (op , rd . to_reg () , base , imm) }
};
}
