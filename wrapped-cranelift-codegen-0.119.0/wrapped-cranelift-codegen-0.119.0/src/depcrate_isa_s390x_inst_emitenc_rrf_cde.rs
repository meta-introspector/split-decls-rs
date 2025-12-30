// Generated macro for enc_rrf_cde (function)
macro_rules! Depcrate_isa_s390x_inst_emitenc_rrf_cde {
() => {
// Module: crate::isa::s390x::inst::emit
// Provides: {"enc_rrf_cde"}
// Dependencies: {}
# [doc = " RRFc/d/e-type instructions."] # [doc = ""] # [doc = "   31     15 11 7  3"] # [doc = "   opcode m3 m4 r1 r2"] # [doc = "       16 12  8  4  0"] # [doc = ""] fn enc_rrf_cde (opcode : u16 , r1 : Reg , r2 : Reg , m3 : u8 , m4 : u8) -> [u8 ; 4] { let mut enc : [u8 ; 4] = [0 ; 4] ; let opcode1 = ((opcode >> 8) & 0xff) as u8 ; let opcode2 = (opcode & 0xff) as u8 ; let r1 = machreg_to_gpr_or_fpr (r1) & 0x0f ; let r2 = machreg_to_gpr_or_fpr (r2) & 0x0f ; let m3 = m3 & 0x0f ; let m4 = m4 & 0x0f ; enc [0] = opcode1 ; enc [1] = opcode2 ; enc [2] = m3 << 4 | m4 ; enc [3] = r1 << 4 | r2 ; enc }
};
}
