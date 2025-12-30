// Generated macro for pretty_print_vreg_element (function)
macro_rules! Depcrate_isa_aarch64_inst_regspretty_print_vreg_element {
() => {
// Module: crate::isa::aarch64::inst::regs
// Provides: {"pretty_print_vreg_element"}
// Dependencies: {}
pub fn pretty_print_vreg_element (reg : Reg , idx : usize , size : ScalarSize) -> String { show_vreg_element (reg , idx as u8 , size) }
};
}
