// Generated macro for one_way_jmp (function)
macro_rules! Depcrate_isa_x64_inst_emitone_way_jmp {
() => {
// Module: crate::isa::x64::inst::emit
// Provides: {"one_way_jmp"}
// Dependencies: {}
# [doc = " Emits a one way conditional jump if CC is set (true)."] fn one_way_jmp (sink : & mut MachBuffer < Inst > , cc : CC , label : MachLabel) { let cond_start = sink . cur_offset () ; let cond_disp_off = cond_start + 2 ; sink . use_label_at_offset (cond_disp_off , label , LabelUse :: JmpRel32) ; sink . put1 (0x0F) ; sink . put1 (0x80 + cc . get_enc ()) ; sink . put4 (0x0) ; }
};
}
