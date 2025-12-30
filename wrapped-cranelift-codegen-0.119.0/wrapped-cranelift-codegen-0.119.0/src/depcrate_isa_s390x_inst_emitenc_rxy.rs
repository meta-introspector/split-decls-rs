// Generated macro for enc_rxy (function)
macro_rules! Depcrate_isa_s390x_inst_emitenc_rxy {
() => {
// Module: crate::isa::s390x::inst::emit
// Provides: {"enc_rxy"}
// Dependencies: {}
# [doc = " RXY-type instructions."] # [doc = ""] # [doc = "   47      39 35 31 27  15  7"] # [doc = "   opcode1 r1 x2 b2 dl2 dh2 opcode2"] # [doc = "        40 36 32 28  16   8       0"] # [doc = ""] fn enc_rxy (opcode : u16 , r1 : Reg , b2 : Reg , x2 : Reg , d2 : u32) -> [u8 ; 6] { let opcode1 = ((opcode >> 8) & 0xff) as u8 ; let opcode2 = (opcode & 0xff) as u8 ; let r1 = machreg_to_gpr_or_fpr (r1) & 0x0f ; let b2 = machreg_to_gpr (b2) & 0x0f ; let x2 = machreg_to_gpr (x2) & 0x0f ; let dl2_lo = (d2 & 0xff) as u8 ; let dl2_hi = ((d2 >> 8) & 0x0f) as u8 ; let dh2 = ((d2 >> 12) & 0xff) as u8 ; let mut enc : [u8 ; 6] = [0 ; 6] ; enc [0] = opcode1 ; enc [1] = r1 << 4 | x2 ; enc [2] = b2 << 4 | dl2_hi ; enc [3] = dl2_lo ; enc [4] = dh2 ; enc [5] = opcode2 ; enc }
};
}
