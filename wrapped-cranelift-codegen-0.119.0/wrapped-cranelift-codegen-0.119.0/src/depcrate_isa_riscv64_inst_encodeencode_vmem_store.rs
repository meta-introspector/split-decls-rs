// Generated macro for encode_vmem_store (function)
macro_rules! Depcrate_isa_riscv64_inst_encodeencode_vmem_store {
() => {
// Module: crate::isa::riscv64::inst::encode
// Provides: {"encode_vmem_store"}
// Dependencies: {}
# [doc = " Encodes a Vector Mem Unit Stride Load instruction."] # [doc = ""] # [doc = " See: https://github.com/riscv/riscv-v-spec/blob/master/vmem-format.adoc"] # [doc = " TODO: These instructions share opcode space with LOAD-FP and STORE-FP"] pub fn encode_vmem_store (opcode : u32 , vs3 : Reg , width : VecElementWidth , rs1 : Reg , sumop : u32 , masking : VecOpMasking , mop : u32 , nf : u32 ,) -> u32 { encode_vmem_load (opcode , vs3 , width , rs1 , sumop , masking , mop , nf) }
};
}
