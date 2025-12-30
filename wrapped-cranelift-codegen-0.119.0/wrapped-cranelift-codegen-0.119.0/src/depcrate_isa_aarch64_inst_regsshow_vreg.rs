// Generated macro for show_vreg (function)
macro_rules! Depcrate_isa_aarch64_inst_regsshow_vreg {
() => {
// Module: crate::isa::aarch64::inst::regs
// Provides: {"show_vreg"}
// Dependencies: {}
fn show_vreg (reg : RealReg) -> String { format ! ("v{}" , reg . hw_enc () & 31) }
};
}
