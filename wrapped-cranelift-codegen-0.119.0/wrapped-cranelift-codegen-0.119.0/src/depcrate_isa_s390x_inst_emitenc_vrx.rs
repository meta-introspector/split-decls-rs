// Generated macro for enc_vrx (function)
macro_rules! Depcrate_isa_s390x_inst_emitenc_vrx {
() => {
// Module: crate::isa::s390x::inst::emit
// Provides: {"enc_vrx"}
// Dependencies: {}
# [doc = " VRX-type instructions."] # [doc = ""] # [doc = "   47      39 35 31 27 15 11  7"] # [doc = "   opcode1 v1 x2 b2 d2 m3 rxb opcode2"] # [doc = "        40 36 32 28 16 12   8       0"] # [doc = ""] fn enc_vrx (opcode : u16 , v1 : Reg , b2 : Reg , x2 : Reg , d2 : u32 , m3 : u8) -> [u8 ; 6] { let opcode1 = ((opcode >> 8) & 0xff) as u8 ; let opcode2 = (opcode & 0xff) as u8 ; let rxb = rxb (Some (v1) , None , None , None) ; let v1 = machreg_to_vr (v1) & 0x0f ; let b2 = machreg_to_gpr (b2) & 0x0f ; let x2 = machreg_to_gpr (x2) & 0x0f ; let d2_lo = (d2 & 0xff) as u8 ; let d2_hi = ((d2 >> 8) & 0x0f) as u8 ; let m3 = m3 & 0x0f ; let mut enc : [u8 ; 6] = [0 ; 6] ; enc [0] = opcode1 ; enc [1] = v1 << 4 | x2 ; enc [2] = b2 << 4 | d2_hi ; enc [3] = d2_lo ; enc [4] = m3 << 4 | rxb ; enc [5] = opcode2 ; enc }
};
}
