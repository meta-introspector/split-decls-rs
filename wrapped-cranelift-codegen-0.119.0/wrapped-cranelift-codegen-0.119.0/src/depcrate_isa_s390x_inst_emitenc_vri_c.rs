// Generated macro for enc_vri_c (function)
macro_rules! Depcrate_isa_s390x_inst_emitenc_vri_c {
() => {
// Module: crate::isa::s390x::inst::emit
// Provides: {"enc_vri_c"}
// Dependencies: {}
# [doc = " VRIc-type instructions."] # [doc = ""] # [doc = "   47      39 35 31 15 11  7"] # [doc = "   opcode1 v1 v3 i2 m4 rxb opcode2"] # [doc = "        40 36 32 16 12   8       0"] # [doc = ""] fn enc_vri_c (opcode : u16 , v1 : Reg , i2 : u16 , v3 : Reg , m4 : u8) -> [u8 ; 6] { let opcode1 = ((opcode >> 8) & 0xff) as u8 ; let opcode2 = (opcode & 0xff) as u8 ; let rxb = rxb (Some (v1) , Some (v3) , None , None) ; let v1 = machreg_to_vr (v1) & 0x0f ; let v3 = machreg_to_vr (v3) & 0x0f ; let m4 = m4 & 0x0f ; let mut enc : [u8 ; 6] = [0 ; 6] ; enc [0] = opcode1 ; enc [1] = v1 << 4 | v3 ; enc [2 .. 4] . copy_from_slice (& i2 . to_be_bytes ()) ; enc [4] = m4 << 4 | rxb ; enc [5] = opcode2 ; enc }
};
}
