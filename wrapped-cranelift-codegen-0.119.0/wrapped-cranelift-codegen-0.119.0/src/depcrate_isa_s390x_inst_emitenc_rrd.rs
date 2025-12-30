// Generated macro for enc_rrd (function)
macro_rules! Depcrate_isa_s390x_inst_emitenc_rrd {
() => {
// Module: crate::isa::s390x::inst::emit
// Provides: {"enc_rrd"}
// Dependencies: {}
# [doc = " RRD-type instructions."] # [doc = ""] # [doc = "   31     15 11 7  3"] # [doc = "   opcode r1 -- r3 r2"] # [doc = "       16 12  8 4  0"] # [doc = ""] fn enc_rrd (opcode : u16 , r1 : Reg , r2 : Reg , r3 : Reg) -> [u8 ; 4] { let mut enc : [u8 ; 4] = [0 ; 4] ; let opcode1 = ((opcode >> 8) & 0xff) as u8 ; let opcode2 = (opcode & 0xff) as u8 ; let r1 = machreg_to_fpr (r1) & 0x0f ; let r2 = machreg_to_fpr (r2) & 0x0f ; let r3 = machreg_to_fpr (r3) & 0x0f ; enc [0] = opcode1 ; enc [1] = opcode2 ; enc [2] = r1 << 4 ; enc [3] = r3 << 4 | r2 ; enc }
};
}
