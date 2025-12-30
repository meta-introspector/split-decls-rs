// Generated macro for enc_vrr_c (function)
macro_rules! Depcrate_isa_s390x_inst_emitenc_vrr_c {
() => {
// Module: crate::isa::s390x::inst::emit
// Provides: {"enc_vrr_c"}
// Dependencies: {}
# [doc = " VRRc-type instructions."] # [doc = ""] # [doc = "   47      39 35 31 27 23 19 15 11  7"] # [doc = "   opcode1 v1 v2 v3 -  m6 m5 m4 rxb opcode2"] # [doc = "        40 36 32 28 24 20 16 12   8       0"] # [doc = ""] fn enc_vrr_c (opcode : u16 , v1 : Reg , v2 : Reg , v3 : Reg , m4 : u8 , m5 : u8 , m6 : u8) -> [u8 ; 6] { let opcode1 = ((opcode >> 8) & 0xff) as u8 ; let opcode2 = (opcode & 0xff) as u8 ; let rxb = rxb (Some (v1) , Some (v2) , Some (v3) , None) ; let v1 = machreg_to_vr (v1) & 0x0f ; let v2 = machreg_to_vr (v2) & 0x0f ; let v3 = machreg_to_vr (v3) & 0x0f ; let m4 = m4 & 0x0f ; let m5 = m5 & 0x0f ; let m6 = m6 & 0x0f ; let mut enc : [u8 ; 6] = [0 ; 6] ; enc [0] = opcode1 ; enc [1] = v1 << 4 | v2 ; enc [2] = v3 << 4 ; enc [3] = m6 << 4 | m5 ; enc [4] = m4 << 4 | rxb ; enc [5] = opcode2 ; enc }
};
}
