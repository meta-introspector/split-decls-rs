// Generated macro for enc_fcmp (function)
macro_rules! Depcrate_isa_aarch64_inst_emitenc_fcmp {
() => {
// Module: crate::isa::aarch64::inst::emit
// Provides: {"enc_fcmp"}
// Dependencies: {}
fn enc_fcmp (size : ScalarSize , rn : Reg , rm : Reg) -> u32 { 0b000_11110_00_1_00000_00_1000_00000_00000 | (size . ftype () << 22) | (machreg_to_vec (rm) << 16) | (machreg_to_vec (rn) << 5) }
};
}
