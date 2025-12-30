// Generated macro for enc_rie_f (function)
macro_rules! Depcrate_isa_s390x_inst_emitenc_rie_f {
() => {
// Module: crate::isa::s390x::inst::emit
// Provides: {"enc_rie_f"}
// Dependencies: {}
# [doc = " RIEf-type instructions."] # [doc = ""] # [doc = "   47      39 35 31 23 15 7"] # [doc = "   opcode1 r1 r2 i3 i4 i5 opcode2"] # [doc = "        40 36 32 24 16  8       0"] # [doc = ""] fn enc_rie_f (opcode : u16 , r1 : Reg , r2 : Reg , i3 : u8 , i4 : u8 , i5 : u8) -> [u8 ; 6] { let mut enc : [u8 ; 6] = [0 ; 6] ; let opcode1 = ((opcode >> 8) & 0xff) as u8 ; let opcode2 = (opcode & 0xff) as u8 ; let r1 = machreg_to_gpr (r1) & 0x0f ; let r2 = machreg_to_gpr (r2) & 0x0f ; enc [0] = opcode1 ; enc [1] = r1 << 4 | r2 ; enc [2] = i3 ; enc [3] = i4 ; enc [4] = i5 ; enc [5] = opcode2 ; enc }
};
}
