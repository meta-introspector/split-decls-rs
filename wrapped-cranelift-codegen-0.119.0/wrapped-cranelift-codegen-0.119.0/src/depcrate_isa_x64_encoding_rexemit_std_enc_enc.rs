// Generated macro for emit_std_enc_enc (function)
macro_rules! Depcrate_isa_x64_encoding_rexemit_std_enc_enc {
() => {
// Module: crate::isa::x64::encoding::rex
// Provides: {"emit_std_enc_enc"}
// Dependencies: {}
# [doc = " This is the core 'emit' function for instructions that do not reference memory."] # [doc = ""] # [doc = " This is conceptually the same as emit_modrm_sib_enc_ge, except it is for the case where the E"] # [doc = " operand is a register rather than memory.  Hence it is much simpler."] pub (crate) fn emit_std_enc_enc < BS : ByteSink + ? Sized > (sink : & mut BS , prefixes : LegacyPrefixes , opcodes : u32 , mut num_opcodes : usize , enc_g : u8 , enc_e : u8 , rex : RexFlags ,) { prefixes . emit (sink) ; rex . emit_two_op (sink , enc_g , enc_e) ; while num_opcodes > 0 { num_opcodes -= 1 ; sink . put1 (((opcodes >> (num_opcodes << 3)) & 0xFF) as u8) ; } sink . put1 (encode_modrm (0b11 , enc_g & 7 , enc_e & 7)) ; }
};
}
