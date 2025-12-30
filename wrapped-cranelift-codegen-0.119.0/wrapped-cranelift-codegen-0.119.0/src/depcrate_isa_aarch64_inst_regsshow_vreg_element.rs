// Generated macro for show_vreg_element (function)
macro_rules! Depcrate_isa_aarch64_inst_regsshow_vreg_element {
() => {
// Module: crate::isa::aarch64::inst::regs
// Provides: {"show_vreg_element"}
// Dependencies: {}
# [doc = " Show an indexed vector element."] pub fn show_vreg_element (reg : Reg , idx : u8 , size : ScalarSize) -> String { assert_eq ! (RegClass :: Float , reg . class ()) ; let s = show_reg (reg) ; let suffix = match size { ScalarSize :: Size8 => ".b" , ScalarSize :: Size16 => ".h" , ScalarSize :: Size32 => ".s" , ScalarSize :: Size64 => ".d" , _ => panic ! ("Unexpected vector element size: {size:?}") , } ; format ! ("{s}{suffix}[{idx}]") }
};
}
