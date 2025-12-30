// Generated macro for enc_rr (function)
macro_rules! Depcrate_isa_s390x_inst_emitenc_rr {
() => {
// Module: crate::isa::s390x::inst::emit
// Provides: {"enc_rr"}
// Dependencies: {}
# [doc = " RR-type instructions."] # [doc = ""] # [doc = "   15     7  3"] # [doc = "   opcode r1 r2"] # [doc = "        8  4  0"] # [doc = ""] fn enc_rr (opcode : u16 , r1 : Reg , r2 : Reg) -> [u8 ; 2] { let mut enc : [u8 ; 2] = [0 ; 2] ; let opcode = (opcode & 0xff) as u8 ; let r1 = machreg_to_gpr_or_fpr (r1) & 0x0f ; let r2 = machreg_to_gpr_or_fpr (r2) & 0x0f ; enc [0] = opcode ; enc [1] = r1 << 4 | r2 ; enc }
};
}
