// Generated macro for encode_vcfg_imm (function)
macro_rules! Depcrate_isa_riscv64_inst_encodeencode_vcfg_imm {
() => {
// Module: crate::isa::riscv64::inst::encode
// Provides: {"encode_vcfg_imm"}
// Dependencies: {}
# [doc = " Encodes a Vector CFG Imm instruction."] # [doc = ""] # [doc = " See: https://github.com/riscv/riscv-v-spec/blob/master/vcfg-format.adoc"] pub fn encode_vcfg_imm (opcode : u32 , rd : Reg , imm : UImm5 , vtype : & VType) -> u32 { let mut bits = 0 ; bits |= unsigned_field_width (opcode , 7) ; bits |= reg_to_gpr_num (rd) << 7 ; bits |= VecOpCategory :: OPCFG . encode () << 12 ; bits |= unsigned_field_width (imm . bits () , 5) << 15 ; bits |= unsigned_field_width (vtype . encode () , 10) << 20 ; bits |= 0b11 << 30 ; bits }
};
}
