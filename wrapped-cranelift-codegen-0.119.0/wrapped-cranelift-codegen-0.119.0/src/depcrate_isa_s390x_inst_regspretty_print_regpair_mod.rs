// Generated macro for pretty_print_regpair_mod (function)
macro_rules! Depcrate_isa_s390x_inst_regspretty_print_regpair_mod {
() => {
// Module: crate::isa::s390x::inst::regs
// Provides: {"pretty_print_regpair_mod"}
// Dependencies: {}
pub fn pretty_print_regpair_mod (rd : WritableRegPair , ri : RegPair) -> String { let rd_hi = rd . hi . to_reg () ; let rd_lo = rd . lo . to_reg () ; let ri_hi = ri . hi ; let ri_lo = ri . lo ; if rd_hi == ri_hi { show_reg (rd_hi) } else { format ! ("{}/{}<-{}/{}" , show_reg (rd_hi) , show_reg (rd_lo) , show_reg (ri_hi) , show_reg (ri_lo)) } }
};
}
