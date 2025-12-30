// Generated macro for encode_vmem_load (function)
macro_rules! Depcrate_isa_riscv64_inst_encodeencode_vmem_load {
() => {
// Module: crate::isa::riscv64::inst::encode
// Provides: {"encode_vmem_load"}
// Dependencies: {}
# [doc = " Encodes a Vector Mem Unit Stride Load instruction."] # [doc = ""] # [doc = " See: https://github.com/riscv/riscv-v-spec/blob/master/vmem-format.adoc"] # [doc = " TODO: These instructions share opcode space with LOAD-FP and STORE-FP"] pub fn encode_vmem_load (opcode : u32 , vd : Reg , width : VecElementWidth , rs1 : Reg , lumop : u32 , masking : VecOpMasking , mop : u32 , nf : u32 ,) -> u32 { let width = match width { VecElementWidth :: E8 => 0b000 , VecElementWidth :: E16 => 0b101 , VecElementWidth :: E32 => 0b110 , VecElementWidth :: E64 => 0b111 , } ; let mut bits = 0 ; bits |= unsigned_field_width (opcode , 7) ; bits |= reg_to_gpr_num (vd) << 7 ; bits |= width << 12 ; bits |= reg_to_gpr_num (rs1) << 15 ; bits |= unsigned_field_width (lumop , 5) << 20 ; bits |= masking . encode () << 25 ; bits |= unsigned_field_width (mop , 2) << 26 ; bits |= 0b0 << 28 ; bits |= unsigned_field_width (nf , 3) << 29 ; bits }
};
}
