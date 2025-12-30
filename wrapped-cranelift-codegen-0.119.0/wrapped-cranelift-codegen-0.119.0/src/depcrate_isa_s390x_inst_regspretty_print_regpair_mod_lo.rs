// Generated macro for pretty_print_regpair_mod_lo (function)
macro_rules! Depcrate_isa_s390x_inst_regspretty_print_regpair_mod_lo {
() => {
// Module: crate::isa::s390x::inst::regs
// Provides: {"pretty_print_regpair_mod_lo"}
// Dependencies: {}
pub fn pretty_print_regpair_mod_lo (rd : WritableRegPair , ri : Reg) -> String { let rd_hi = rd . hi . to_reg () ; let rd_lo = rd . lo . to_reg () ; if rd_lo == ri { show_reg (rd_hi) } else { format ! ("{}/{}<-_/{}" , show_reg (rd_hi) , show_reg (rd_lo) , show_reg (ri) ,) } }
};
}
