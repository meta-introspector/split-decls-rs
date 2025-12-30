// Generated macro for encode_zcbmem_store (function)
macro_rules! Depcrate_isa_riscv64_inst_encodeencode_zcbmem_store {
() => {
// Module: crate::isa::riscv64::inst::encode
// Provides: {"encode_zcbmem_store"}
// Dependencies: {}
pub fn encode_zcbmem_store (op : ZcbMemOp , src : Reg , base : Reg , imm : Uimm2) -> u16 { encode_zcbmem_bits (op , src , base , imm) }
};
}
