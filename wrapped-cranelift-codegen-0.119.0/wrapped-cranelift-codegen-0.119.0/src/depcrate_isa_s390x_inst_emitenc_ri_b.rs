// Generated macro for enc_ri_b (function)
macro_rules! Depcrate_isa_s390x_inst_emitenc_ri_b {
() => {
// Module: crate::isa::s390x::inst::emit
// Provides: {"enc_ri_b"}
// Dependencies: {}
# [doc = " RIb-type instructions."] # [doc = ""] # [doc = "   31      23 19      15"] # [doc = "   opcode1 r1 opcode2 ri2"] # [doc = "        24 20      16   0"] # [doc = ""] fn enc_ri_b (opcode : u16 , r1 : Reg , ri2 : i32) -> [u8 ; 4] { let mut enc : [u8 ; 4] = [0 ; 4] ; let opcode1 = ((opcode >> 4) & 0xff) as u8 ; let opcode2 = (opcode & 0xf) as u8 ; let r1 = machreg_to_gpr (r1) & 0x0f ; let ri2 = ((ri2 >> 1) & 0xffff) as u16 ; enc [0] = opcode1 ; enc [1] = r1 << 4 | opcode2 ; enc [2 ..] . copy_from_slice (& ri2 . to_be_bytes ()) ; enc }
};
}
