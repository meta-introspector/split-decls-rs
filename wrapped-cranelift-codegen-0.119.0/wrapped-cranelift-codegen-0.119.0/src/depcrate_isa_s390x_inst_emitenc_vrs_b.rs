// Generated macro for enc_vrs_b (function)
macro_rules! Depcrate_isa_s390x_inst_emitenc_vrs_b {
() => {
// Module: crate::isa::s390x::inst::emit
// Provides: {"enc_vrs_b"}
// Dependencies: {}
# [doc = " VRSb-type instructions."] # [doc = ""] # [doc = "   47      39 35 31 27 15 11  7"] # [doc = "   opcode1 v1 r3 b2 d2 m4 rxb opcode2"] # [doc = "        40 36 32 28 16 12   8       0"] # [doc = ""] fn enc_vrs_b (opcode : u16 , v1 : Reg , b2 : Reg , d2 : u32 , r3 : Reg , m4 : u8) -> [u8 ; 6] { let opcode1 = ((opcode >> 8) & 0xff) as u8 ; let opcode2 = (opcode & 0xff) as u8 ; let rxb = rxb (Some (v1) , None , None , None) ; let v1 = machreg_to_vr (v1) & 0x0f ; let b2 = machreg_to_gpr (b2) & 0x0f ; let r3 = machreg_to_gpr (r3) & 0x0f ; let d2_lo = (d2 & 0xff) as u8 ; let d2_hi = ((d2 >> 8) & 0x0f) as u8 ; let m4 = m4 & 0x0f ; let mut enc : [u8 ; 6] = [0 ; 6] ; enc [0] = opcode1 ; enc [1] = v1 << 4 | r3 ; enc [2] = b2 << 4 | d2_hi ; enc [3] = d2_lo ; enc [4] = m4 << 4 | rxb ; enc [5] = opcode2 ; enc }
};
}
