// Generated macro for enc_br (function)
macro_rules! Depcrate_isa_aarch64_inst_emitenc_br {
() => {
// Module: crate::isa::aarch64::inst::emit
// Provides: {"enc_br"}
// Dependencies: {}
pub (crate) fn enc_br (rn : Reg) -> u32 { 0b1101011_0000_11111_000000_00000_00000 | (machreg_to_gpr (rn) << 5) }
};
}
