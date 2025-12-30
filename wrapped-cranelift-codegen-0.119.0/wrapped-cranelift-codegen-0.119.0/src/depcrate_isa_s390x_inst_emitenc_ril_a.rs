// Generated macro for enc_ril_a (function)
macro_rules! Depcrate_isa_s390x_inst_emitenc_ril_a {
() => {
// Module: crate::isa::s390x::inst::emit
// Provides: {"enc_ril_a"}
// Dependencies: {}
# [doc = " RILa-type instructions."] # [doc = ""] # [doc = "   47      39 35      31"] # [doc = "   opcode1 r1 opcode2 i2"] # [doc = "        40 36      32  0"] # [doc = ""] fn enc_ril_a (opcode : u16 , r1 : Reg , i2 : u32) -> [u8 ; 6] { let mut enc : [u8 ; 6] = [0 ; 6] ; let opcode1 = ((opcode >> 4) & 0xff) as u8 ; let opcode2 = (opcode & 0xf) as u8 ; let r1 = machreg_to_gpr (r1) & 0x0f ; enc [0] = opcode1 ; enc [1] = r1 << 4 | opcode2 ; enc [2 ..] . copy_from_slice (& i2 . to_be_bytes ()) ; enc }
};
}
