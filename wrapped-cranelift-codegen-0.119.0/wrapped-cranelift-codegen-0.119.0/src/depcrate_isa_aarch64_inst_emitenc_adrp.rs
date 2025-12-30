// Generated macro for enc_adrp (function)
macro_rules! Depcrate_isa_aarch64_inst_emitenc_adrp {
() => {
// Module: crate::isa::aarch64::inst::emit
// Provides: {"enc_adrp"}
// Dependencies: {}
pub (crate) fn enc_adrp (off : i32 , rd : Writable < Reg >) -> u32 { let opcode = 0b10010000 << 24 ; enc_adr_inst (opcode , off , rd) }
};
}
