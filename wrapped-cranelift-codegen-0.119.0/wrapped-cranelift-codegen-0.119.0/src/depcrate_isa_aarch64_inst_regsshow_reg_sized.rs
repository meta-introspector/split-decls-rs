// Generated macro for show_reg_sized (function)
macro_rules! Depcrate_isa_aarch64_inst_regsshow_reg_sized {
() => {
// Module: crate::isa::aarch64::inst::regs
// Provides: {"show_reg_sized"}
// Dependencies: {}
fn show_reg_sized (reg : Reg , size : OperandSize) -> String { match reg . class () { RegClass :: Int => show_ireg_sized (reg , size) , RegClass :: Float => show_reg (reg) , RegClass :: Vector => unreachable ! () , } }
};
}
