// Generated macro for show_ireg (function)
macro_rules! Depcrate_isa_aarch64_inst_regsshow_ireg {
() => {
// Module: crate::isa::aarch64::inst::regs
// Provides: {"show_ireg"}
// Dependencies: {}
fn show_ireg (reg : RealReg) -> String { match reg . hw_enc () { 29 => "fp" . to_string () , 30 => "lr" . to_string () , 31 => "xzr" . to_string () , 63 => "sp" . to_string () , x => { debug_assert ! (x < 29) ; format ! ("x{x}") } } }
};
}
