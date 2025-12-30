// Generated macro for enc_adr (function)
macro_rules! Depcrate_isa_aarch64_inst_emitenc_adr {
() => {
// Module: crate::isa::aarch64::inst::emit
// Provides: {"enc_adr"}
// Dependencies: {}
pub (crate) fn enc_adr (off : i32 , rd : Writable < Reg >) -> u32 { let opcode = 0b00010000 << 24 ; enc_adr_inst (opcode , off , rd) }
};
}
