// Generated macro for enc_e (function)
macro_rules! Depcrate_isa_s390x_inst_emitenc_e {
() => {
// Module: crate::isa::s390x::inst::emit
// Provides: {"enc_e"}
// Dependencies: {}
# [doc = " E-type instructions."] # [doc = ""] # [doc = "   15"] # [doc = "   opcode"] # [doc = "        0"] # [doc = ""] fn enc_e (opcode : u16) -> [u8 ; 2] { let mut enc : [u8 ; 2] = [0 ; 2] ; let opcode1 = ((opcode >> 8) & 0xff) as u8 ; let opcode2 = (opcode & 0xff) as u8 ; enc [0] = opcode1 ; enc [1] = opcode2 ; enc }
};
}
