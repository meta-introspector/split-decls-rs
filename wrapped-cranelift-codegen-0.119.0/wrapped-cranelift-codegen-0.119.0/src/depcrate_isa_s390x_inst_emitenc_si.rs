// Generated macro for enc_si (function)
macro_rules! Depcrate_isa_s390x_inst_emitenc_si {
() => {
// Module: crate::isa::s390x::inst::emit
// Provides: {"enc_si"}
// Dependencies: {}
# [doc = " SI-type instructions."] # [doc = ""] # [doc = "   31     23 15 11"] # [doc = "   opcode i2 b1 d1"] # [doc = "       24 16 12  0"] # [doc = ""] fn enc_si (opcode : u16 , b1 : Reg , d1 : u32 , i2 : u8) -> [u8 ; 4] { let opcode = (opcode & 0xff) as u8 ; let b1 = machreg_to_gpr (b1) & 0x0f ; let d1_lo = (d1 & 0xff) as u8 ; let d1_hi = ((d1 >> 8) & 0x0f) as u8 ; let mut enc : [u8 ; 4] = [0 ; 4] ; enc [0] = opcode ; enc [1] = i2 ; enc [2] = b1 << 4 | d1_hi ; enc [3] = d1_lo ; enc }
};
}
