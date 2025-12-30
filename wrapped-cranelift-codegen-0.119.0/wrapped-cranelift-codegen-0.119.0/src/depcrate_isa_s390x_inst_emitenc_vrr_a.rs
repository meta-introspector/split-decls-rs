// Generated macro for enc_vrr_a (function)
macro_rules! Depcrate_isa_s390x_inst_emitenc_vrr_a {
() => {
// Module: crate::isa::s390x::inst::emit
// Provides: {"enc_vrr_a"}
// Dependencies: {}
# [doc = " VRRa-type instructions."] # [doc = ""] # [doc = "   47      39 35 31 23 19 15 11  7"] # [doc = "   opcode1 v1 v2 -  m5 m3 m2 rxb opcode2"] # [doc = "        40 36 32 24 20 16 12   8       0"] # [doc = ""] fn enc_vrr_a (opcode : u16 , v1 : Reg , v2 : Reg , m3 : u8 , m4 : u8 , m5 : u8) -> [u8 ; 6] { let opcode1 = ((opcode >> 8) & 0xff) as u8 ; let opcode2 = (opcode & 0xff) as u8 ; let rxb = rxb (Some (v1) , Some (v2) , None , None) ; let v1 = machreg_to_vr (v1) & 0x0f ; let v2 = machreg_to_vr (v2) & 0x0f ; let m3 = m3 & 0x0f ; let m4 = m4 & 0x0f ; let m5 = m5 & 0x0f ; let mut enc : [u8 ; 6] = [0 ; 6] ; enc [0] = opcode1 ; enc [1] = v1 << 4 | v2 ; enc [2] = 0 ; enc [3] = m5 << 4 | m4 ; enc [4] = m3 << 4 | rxb ; enc [5] = opcode2 ; enc }
};
}
