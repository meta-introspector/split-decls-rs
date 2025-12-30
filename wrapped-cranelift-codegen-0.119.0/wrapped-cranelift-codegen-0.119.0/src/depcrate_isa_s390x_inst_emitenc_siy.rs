// Generated macro for enc_siy (function)
macro_rules! Depcrate_isa_s390x_inst_emitenc_siy {
() => {
// Module: crate::isa::s390x::inst::emit
// Provides: {"enc_siy"}
// Dependencies: {}
# [doc = " SIY-type instructions."] # [doc = ""] # [doc = "   47      39 31 27  15  7"] # [doc = "   opcode1 i2 b1 dl1 dh1 opcode2"] # [doc = "        40 32 28  16   8       0"] # [doc = ""] fn enc_siy (opcode : u16 , b1 : Reg , d1 : u32 , i2 : u8) -> [u8 ; 6] { let opcode1 = ((opcode >> 8) & 0xff) as u8 ; let opcode2 = (opcode & 0xff) as u8 ; let b1 = machreg_to_gpr (b1) & 0x0f ; let dl1_lo = (d1 & 0xff) as u8 ; let dl1_hi = ((d1 >> 8) & 0x0f) as u8 ; let dh1 = ((d1 >> 12) & 0xff) as u8 ; let mut enc : [u8 ; 6] = [0 ; 6] ; enc [0] = opcode1 ; enc [1] = i2 ; enc [2] = b1 << 4 | dl1_hi ; enc [3] = dl1_lo ; enc [4] = dh1 ; enc [5] = opcode2 ; enc }
};
}
