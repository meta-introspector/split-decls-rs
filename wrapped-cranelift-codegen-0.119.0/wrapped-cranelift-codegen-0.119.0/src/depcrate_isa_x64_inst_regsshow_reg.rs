// Generated macro for show_reg (function)
macro_rules! Depcrate_isa_x64_inst_regsshow_reg {
() => {
// Module: crate::isa::x64::inst::regs
// Provides: {"show_reg"}
// Dependencies: {}
pub fn show_reg (reg : Reg) -> String { if let Some (rreg) = reg . to_real_reg () { realreg_name (rreg) . to_string () } else { format ! ("%{reg:?}") } }
};
}
