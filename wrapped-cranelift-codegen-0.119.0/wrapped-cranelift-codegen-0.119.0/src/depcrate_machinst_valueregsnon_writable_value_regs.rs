// Generated macro for non_writable_value_regs (function)
macro_rules! Depcrate_machinst_valueregsnon_writable_value_regs {
() => {
// Module: crate::machinst::valueregs
// Provides: {"non_writable_value_regs"}
// Dependencies: {}
# [doc = " Strip a writable ValueRegs down to a readonly ValueRegs."] # [allow (dead_code)] pub (crate) fn non_writable_value_regs (regs : ValueRegs < Writable < Reg > >) -> ValueRegs < Reg > { regs . map (| r | r . to_reg ()) }
};
}
