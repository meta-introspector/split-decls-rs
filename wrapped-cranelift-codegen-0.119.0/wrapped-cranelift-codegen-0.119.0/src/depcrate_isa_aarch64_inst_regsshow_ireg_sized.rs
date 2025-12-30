// Generated macro for show_ireg_sized (function)
macro_rules! Depcrate_isa_aarch64_inst_regsshow_ireg_sized {
() => {
// Module: crate::isa::aarch64::inst::regs
// Provides: {"show_ireg_sized"}
// Dependencies: {}
# [doc = " If `ireg` denotes an Int-classed reg, make a best-effort attempt to show"] # [doc = " its name at the 32-bit size."] pub fn show_ireg_sized (reg : Reg , size : OperandSize) -> String { let mut s = show_reg (reg) ; if reg . class () != RegClass :: Int || ! size . is32 () { return s ; } if reg . class () == RegClass :: Int && size . is32 () && s . starts_with ("x") { s = "w" . to_string () + & s [1 ..] ; } s }
};
}
