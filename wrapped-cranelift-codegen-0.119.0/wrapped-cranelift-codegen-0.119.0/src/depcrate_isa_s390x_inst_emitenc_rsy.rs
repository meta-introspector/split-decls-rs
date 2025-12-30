// Generated macro for enc_rsy (function)
macro_rules! Depcrate_isa_s390x_inst_emitenc_rsy {
() => {
// Module: crate::isa::s390x::inst::emit
// Provides: {"enc_rsy"}
// Dependencies: {}
# [doc = " RSY-type instructions."] # [doc = ""] # [doc = "   47      39 35 31 27  15  7"] # [doc = "   opcode1 r1 r3 b2 dl2 dh2 opcode2"] # [doc = "        40 36 32 28  16   8       0"] # [doc = ""] fn enc_rsy (opcode : u16 , r1 : Reg , r3 : Reg , b2 : Reg , d2 : u32) -> [u8 ; 6] { let opcode1 = ((opcode >> 8) & 0xff) as u8 ; let opcode2 = (opcode & 0xff) as u8 ; let r1 = machreg_to_gpr_or_fpr (r1) & 0x0f ; let r3 = machreg_to_gpr_or_fpr (r3) & 0x0f ; let b2 = machreg_to_gpr (b2) & 0x0f ; let dl2_lo = (d2 & 0xff) as u8 ; let dl2_hi = ((d2 >> 8) & 0x0f) as u8 ; let dh2 = ((d2 >> 12) & 0xff) as u8 ; let mut enc : [u8 ; 6] = [0 ; 6] ; enc [0] = opcode1 ; enc [1] = r1 << 4 | r3 ; enc [2] = b2 << 4 | dl2_hi ; enc [3] = dl2_lo ; enc [4] = dh2 ; enc [5] = opcode2 ; enc }
};
}
