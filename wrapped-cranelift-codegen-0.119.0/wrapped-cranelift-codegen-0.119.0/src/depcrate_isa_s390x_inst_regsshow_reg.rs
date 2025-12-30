// Generated macro for show_reg (function)
macro_rules! Depcrate_isa_s390x_inst_regsshow_reg {
() => {
// Module: crate::isa::s390x::inst::regs
// Provides: {"show_reg"}
// Dependencies: {}
pub fn show_reg (reg : Reg) -> String { if let Some (rreg) = reg . to_real_reg () { match rreg . class () { RegClass :: Int => format ! ("%r{}" , rreg . hw_enc ()) , RegClass :: Float => format ! ("%v{}" , rreg . hw_enc ()) , RegClass :: Vector => unreachable ! () , } } else { format ! ("%{reg:?}") } }
};
}
