// Generated macro for enc_adr_inst (function)
macro_rules! Depcrate_isa_aarch64_inst_emitenc_adr_inst {
() => {
// Module: crate::isa::aarch64::inst::emit
// Provides: {"enc_adr_inst"}
// Dependencies: {}
pub (crate) fn enc_adr_inst (opcode : u32 , off : i32 , rd : Writable < Reg >) -> u32 { let off = u32 :: try_from (off) . unwrap () ; let immlo = off & 3 ; let immhi = (off >> 2) & ((1 << 19) - 1) ; opcode | (immlo << 29) | (immhi << 5) | machreg_to_gpr (rd . to_reg ()) }
};
}
