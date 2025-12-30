// Generated macro for pretty_print_reg_mod (function)
macro_rules! Depcrate_isa_s390x_inst_regspretty_print_reg_mod {
() => {
// Module: crate::isa::s390x::inst::regs
// Provides: {"pretty_print_reg_mod"}
// Dependencies: {}
pub fn pretty_print_reg_mod (rd : Writable < Reg > , ri : Reg) -> String { let output = rd . to_reg () ; let input = ri ; if output == input { show_reg (output) } else { format ! ("{}<-{}" , show_reg (output) , show_reg (input)) } }
};
}
