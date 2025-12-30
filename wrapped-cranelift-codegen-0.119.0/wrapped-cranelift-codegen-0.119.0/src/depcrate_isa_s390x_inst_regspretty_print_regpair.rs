// Generated macro for pretty_print_regpair (function)
macro_rules! Depcrate_isa_s390x_inst_regspretty_print_regpair {
() => {
// Module: crate::isa::s390x::inst::regs
// Provides: {"pretty_print_regpair"}
// Dependencies: {}
pub fn pretty_print_regpair (pair : RegPair) -> String { let hi = pair . hi ; let lo = pair . lo ; if let Some (hi_reg) = hi . to_real_reg () { if let Some (lo_reg) = lo . to_real_reg () { assert ! (hi_reg . hw_enc () + 1 == lo_reg . hw_enc () , "Invalid regpair: {} {}" , show_reg (hi) , show_reg (lo)) ; return show_reg (hi) ; } } format ! ("{}/{}" , show_reg (hi) , show_reg (lo)) }
};
}
