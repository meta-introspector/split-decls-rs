// Generated macro for show_vreg_vector (function)
macro_rules! Depcrate_isa_aarch64_inst_regsshow_vreg_vector {
() => {
// Module: crate::isa::aarch64::inst::regs
// Provides: {"show_vreg_vector"}
// Dependencies: {}
# [doc = " Show a vector register."] pub fn show_vreg_vector (reg : Reg , size : VectorSize) -> String { assert_eq ! (RegClass :: Float , reg . class ()) ; let mut s = show_reg (reg) ; let suffix = match size { VectorSize :: Size8x8 => ".8b" , VectorSize :: Size8x16 => ".16b" , VectorSize :: Size16x4 => ".4h" , VectorSize :: Size16x8 => ".8h" , VectorSize :: Size32x2 => ".2s" , VectorSize :: Size32x4 => ".4s" , VectorSize :: Size64x2 => ".2d" , } ; s . push_str (suffix) ; s }
};
}
