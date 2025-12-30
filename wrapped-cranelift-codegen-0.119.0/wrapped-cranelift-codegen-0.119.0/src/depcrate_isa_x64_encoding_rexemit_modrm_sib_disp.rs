// Generated macro for emit_modrm_sib_disp (function)
macro_rules! Depcrate_isa_x64_encoding_rexemit_modrm_sib_disp {
() => {
// Module: crate::isa::x64::encoding::rex
// Provides: {"emit_modrm_sib_disp"}
// Dependencies: {}
pub (crate) fn emit_modrm_sib_disp (sink : & mut MachBuffer < Inst > , enc_g : u8 , mem_e : & Amode , bytes_at_end : u8 , evex_scaling : Option < i8 > ,) { match * mem_e { Amode :: ImmReg { simm32 , base , .. } => { let enc_e = int_reg_enc (base) ; let mut imm = Imm :: new (simm32 , evex_scaling) ; let enc_e_low3 = enc_e & 7 ; if enc_e_low3 != regs :: ENC_RSP { if enc_e_low3 == regs :: ENC_RBP { imm . force_immediate () ; } sink . put1 (encode_modrm (imm . m0d () , enc_g & 7 , enc_e & 7)) ; imm . emit (sink) ; } else { sink . put1 (encode_modrm (imm . m0d () , enc_g & 7 , 0b100)) ; sink . put1 (0b00_100_100) ; imm . emit (sink) ; } } Amode :: ImmRegRegShift { simm32 , base : reg_base , index : reg_index , shift , .. } => { let enc_base = int_reg_enc (* reg_base) ; let enc_index = int_reg_enc (* reg_index) ; assert ! (enc_index != regs :: ENC_RSP) ; let mut imm = Imm :: new (simm32 , evex_scaling) ; if enc_base & 7 == regs :: ENC_RBP { imm . force_immediate () ; } sink . put1 (encode_modrm (imm . m0d () , enc_g & 7 , 0b100)) ; sink . put1 (encode_sib (shift , enc_index & 7 , enc_base & 7)) ; imm . emit (sink) ; } Amode :: RipRelative { ref target } => { sink . put1 (encode_modrm (0b00 , enc_g & 7 , 0b101)) ; let offset = sink . cur_offset () ; sink . use_label_at_offset (offset , * target , LabelUse :: JmpRel32) ; sink . put4 (- (i32 :: from (bytes_at_end)) as u32) ; } } }
};
}
