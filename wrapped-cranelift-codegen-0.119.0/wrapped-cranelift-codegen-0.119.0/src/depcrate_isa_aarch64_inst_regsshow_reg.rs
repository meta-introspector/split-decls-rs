// Generated macro for show_reg (function)
macro_rules! Depcrate_isa_aarch64_inst_regsshow_reg {
() => {
// Module: crate::isa::aarch64::inst::regs
// Provides: {"show_reg"}
// Dependencies: {}
fn show_reg (reg : Reg) -> String { if let Some (rreg) = reg . to_real_reg () { match rreg . class () { RegClass :: Int => show_ireg (rreg) , RegClass :: Float => show_vreg (rreg) , RegClass :: Vector => unreachable ! () , } } else { format ! ("%{reg:?}") } }
};
}
