// Generated macro for enc_fcsel (function)
macro_rules! Depcrate_isa_aarch64_inst_emitenc_fcsel {
() => {
// Module: crate::isa::aarch64::inst::emit
// Provides: {"enc_fcsel"}
// Dependencies: {}
fn enc_fcsel (rd : Writable < Reg > , rn : Reg , rm : Reg , cond : Cond , size : ScalarSize) -> u32 { 0b000_11110_00_1_00000_0000_11_00000_00000 | (size . ftype () << 22) | (machreg_to_vec (rm) << 16) | (machreg_to_vec (rn) << 5) | machreg_to_vec (rd . to_reg ()) | (cond . bits () << 12) }
};
}
