// Generated macro for emit_std_reg_mem (function)
macro_rules! Depcrate_isa_x64_encoding_rexemit_std_reg_mem {
() => {
// Module: crate::isa::x64::encoding::rex
// Provides: {"emit_std_reg_mem"}
// Dependencies: {}
pub (crate) fn emit_std_reg_mem (sink : & mut MachBuffer < Inst > , prefixes : LegacyPrefixes , opcodes : u32 , num_opcodes : usize , reg_g : Reg , mem_e : & Amode , rex : RexFlags , bytes_at_end : u8 ,) { let enc_g = reg_enc (reg_g) ; emit_std_enc_mem (sink , prefixes , opcodes , num_opcodes , enc_g , mem_e , rex , bytes_at_end ,) ; }
};
}
