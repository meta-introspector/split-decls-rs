// Generated macro for pretty_print_fpr (function)
macro_rules! Depcrate_isa_s390x_inst_regspretty_print_fpr {
() => {
// Module: crate::isa::s390x::inst::regs
// Provides: {"pretty_print_fpr"}
// Dependencies: {}
pub fn pretty_print_fpr (reg : Reg) -> (String , Option < String >) { (show_reg (reg) , maybe_show_fpr (reg)) }
};
}
