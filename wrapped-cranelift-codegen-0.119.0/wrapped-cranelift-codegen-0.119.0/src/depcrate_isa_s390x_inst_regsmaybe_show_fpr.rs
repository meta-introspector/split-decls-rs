// Generated macro for maybe_show_fpr (function)
macro_rules! Depcrate_isa_s390x_inst_regsmaybe_show_fpr {
() => {
// Module: crate::isa::s390x::inst::regs
// Provides: {"maybe_show_fpr"}
// Dependencies: {}
pub fn maybe_show_fpr (reg : Reg) -> Option < String > { if let Some (rreg) = reg . to_real_reg () { if is_fpr (reg) { return Some (format ! ("%f{}" , rreg . hw_enc ())) ; } } None }
};
}
