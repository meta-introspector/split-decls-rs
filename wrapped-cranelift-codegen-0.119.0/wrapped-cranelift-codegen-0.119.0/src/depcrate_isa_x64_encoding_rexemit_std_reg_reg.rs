// Generated macro for emit_std_reg_reg (function)
macro_rules! Depcrate_isa_x64_encoding_rexemit_std_reg_reg {
() => {
// Module: crate::isa::x64::encoding::rex
// Provides: {"emit_std_reg_reg"}
// Dependencies: {}
pub (crate) fn emit_std_reg_reg < BS : ByteSink + ? Sized > (sink : & mut BS , prefixes : LegacyPrefixes , opcodes : u32 , num_opcodes : usize , reg_g : Reg , reg_e : Reg , rex : RexFlags ,) { let enc_g = reg_enc (reg_g) ; let enc_e = reg_enc (reg_e) ; emit_std_enc_enc (sink , prefixes , opcodes , num_opcodes , enc_g , enc_e , rex) ; }
};
}
