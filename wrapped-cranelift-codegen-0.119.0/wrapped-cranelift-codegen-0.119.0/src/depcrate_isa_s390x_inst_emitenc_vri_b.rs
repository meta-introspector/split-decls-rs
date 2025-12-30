// Generated macro for enc_vri_b (function)
macro_rules! Depcrate_isa_s390x_inst_emitenc_vri_b {
() => {
// Module: crate::isa::s390x::inst::emit
// Provides: {"enc_vri_b"}
// Dependencies: {}
# [doc = " VRIb-type instructions."] # [doc = ""] # [doc = "   47      39 35 31 23 15 11  7"] # [doc = "   opcode1 v1 -  i2 i3 m4 rxb opcode2"] # [doc = "        40 36 32 24 16 12   8       0"] # [doc = ""] fn enc_vri_b (opcode : u16 , v1 : Reg , i2 : u8 , i3 : u8 , m4 : u8) -> [u8 ; 6] { let opcode1 = ((opcode >> 8) & 0xff) as u8 ; let opcode2 = (opcode & 0xff) as u8 ; let rxb = rxb (Some (v1) , None , None , None) ; let v1 = machreg_to_vr (v1) & 0x0f ; let m4 = m4 & 0x0f ; let mut enc : [u8 ; 6] = [0 ; 6] ; enc [0] = opcode1 ; enc [1] = v1 << 4 ; enc [2] = i2 ; enc [3] = i3 ; enc [4] = m4 << 4 | rxb ; enc [5] = opcode2 ; enc }
};
}
