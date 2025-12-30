// Generated macro for enc_rs (function)
macro_rules! Depcrate_isa_s390x_inst_emitenc_rs {
() => {
// Module: crate::isa::s390x::inst::emit
// Provides: {"enc_rs"}
// Dependencies: {}
# [doc = " RS-type instructions."] # [doc = ""] # [doc = "   31     23 19 15 11"] # [doc = "   opcode r1 r3 b2 d2"] # [doc = "       24 20 16 12  0"] # [doc = ""] fn enc_rs (opcode : u16 , r1 : Reg , r3 : Reg , b2 : Reg , d2 : u32) -> [u8 ; 4] { let opcode = (opcode & 0xff) as u8 ; let r1 = machreg_to_gpr_or_fpr (r1) & 0x0f ; let r3 = machreg_to_gpr_or_fpr (r3) & 0x0f ; let b2 = machreg_to_gpr (b2) & 0x0f ; let d2_lo = (d2 & 0xff) as u8 ; let d2_hi = ((d2 >> 8) & 0x0f) as u8 ; let mut enc : [u8 ; 4] = [0 ; 4] ; enc [0] = opcode ; enc [1] = r1 << 4 | r3 ; enc [2] = b2 << 4 | d2_hi ; enc [3] = d2_lo ; enc }
};
}
