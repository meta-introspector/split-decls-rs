// Generated macro for enc_vrr_f (function)
macro_rules! Depcrate_isa_s390x_inst_emitenc_vrr_f {
() => {
// Module: crate::isa::s390x::inst::emit
// Provides: {"enc_vrr_f"}
// Dependencies: {}
# [doc = " VRRf-type instructions."] # [doc = ""] # [doc = "   47      39 35 31 27 11  7"] # [doc = "   opcode1 v1 r2 r3 -  rxb opcode2"] # [doc = "        40 36 32 28 12   8       0"] # [doc = ""] fn enc_vrr_f (opcode : u16 , v1 : Reg , r2 : Reg , r3 : Reg) -> [u8 ; 6] { let opcode1 = ((opcode >> 8) & 0xff) as u8 ; let opcode2 = (opcode & 0xff) as u8 ; let rxb = rxb (Some (v1) , None , None , None) ; let v1 = machreg_to_vr (v1) & 0x0f ; let r2 = machreg_to_gpr (r2) & 0x0f ; let r3 = machreg_to_gpr (r3) & 0x0f ; let mut enc : [u8 ; 6] = [0 ; 6] ; enc [0] = opcode1 ; enc [1] = v1 << 4 | r2 ; enc [2] = r3 << 4 ; enc [4] = rxb ; enc [5] = opcode2 ; enc }
};
}
