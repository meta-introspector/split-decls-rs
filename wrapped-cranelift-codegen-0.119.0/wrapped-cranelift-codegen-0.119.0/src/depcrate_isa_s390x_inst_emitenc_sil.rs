// Generated macro for enc_sil (function)
macro_rules! Depcrate_isa_s390x_inst_emitenc_sil {
() => {
// Module: crate::isa::s390x::inst::emit
// Provides: {"enc_sil"}
// Dependencies: {}
# [doc = " SIL-type instructions."] # [doc = ""] # [doc = "   47     31 27 15"] # [doc = "   opcode b1 d1 i2"] # [doc = "       32 28 16  0"] # [doc = ""] fn enc_sil (opcode : u16 , b1 : Reg , d1 : u32 , i2 : i16) -> [u8 ; 6] { let opcode1 = ((opcode >> 8) & 0xff) as u8 ; let opcode2 = (opcode & 0xff) as u8 ; let b1 = machreg_to_gpr (b1) & 0x0f ; let d1_lo = (d1 & 0xff) as u8 ; let d1_hi = ((d1 >> 8) & 0x0f) as u8 ; let mut enc : [u8 ; 6] = [0 ; 6] ; enc [0] = opcode1 ; enc [1] = opcode2 ; enc [2] = b1 << 4 | d1_hi ; enc [3] = d1_lo ; enc [4 ..] . copy_from_slice (& i2 . to_be_bytes ()) ; enc }
};
}
