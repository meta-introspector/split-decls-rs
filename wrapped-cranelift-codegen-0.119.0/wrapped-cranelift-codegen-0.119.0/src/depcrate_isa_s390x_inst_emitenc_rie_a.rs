// Generated macro for enc_rie_a (function)
macro_rules! Depcrate_isa_s390x_inst_emitenc_rie_a {
() => {
// Module: crate::isa::s390x::inst::emit
// Provides: {"enc_rie_a"}
// Dependencies: {}
# [doc = " RIEa-type instructions."] # [doc = ""] # [doc = "   47      39 35 31 15 11 7"] # [doc = "   opcode1 r1 -- i2 m3 -- opcode2"] # [doc = "        40 36 32 16 12 8       0"] # [doc = ""] fn enc_rie_a (opcode : u16 , r1 : Reg , i2 : u16 , m3 : u8) -> [u8 ; 6] { let mut enc : [u8 ; 6] = [0 ; 6] ; let opcode1 = ((opcode >> 8) & 0xff) as u8 ; let opcode2 = (opcode & 0xff) as u8 ; let r1 = machreg_to_gpr (r1) & 0x0f ; let m3 = m3 & 0x0f ; enc [0] = opcode1 ; enc [1] = r1 << 4 ; enc [2 .. 4] . copy_from_slice (& i2 . to_be_bytes ()) ; enc [4] = m3 << 4 ; enc [5] = opcode2 ; enc }
};
}
