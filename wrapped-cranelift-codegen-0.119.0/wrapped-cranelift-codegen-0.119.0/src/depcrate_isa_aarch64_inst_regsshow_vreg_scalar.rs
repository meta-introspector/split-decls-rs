// Generated macro for show_vreg_scalar (function)
macro_rules! Depcrate_isa_aarch64_inst_regsshow_vreg_scalar {
() => {
// Module: crate::isa::aarch64::inst::regs
// Provides: {"show_vreg_scalar"}
// Dependencies: {}
# [doc = " Show a vector register used in a scalar context."] pub fn show_vreg_scalar (reg : Reg , size : ScalarSize) -> String { let mut s = show_reg (reg) ; if reg . class () != RegClass :: Float { return s ; } if s . starts_with ("v") { let replacement = match size { ScalarSize :: Size8 => "b" , ScalarSize :: Size16 => "h" , ScalarSize :: Size32 => "s" , ScalarSize :: Size64 => "d" , ScalarSize :: Size128 => "q" , } ; s . replace_range (0 .. 1 , replacement) ; } s }
};
}
