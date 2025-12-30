// Generated macro for writable_value_regs (function)
macro_rules! Depcrate_machinst_valueregswritable_value_regs {
() => {
// Module: crate::machinst::valueregs
// Provides: {"writable_value_regs"}
// Dependencies: {}
# [doc = " Create a writable ValueRegs."] # [allow (dead_code)] pub (crate) fn writable_value_regs (regs : ValueRegs < Reg >) -> ValueRegs < Writable < Reg > > { regs . map (| r | Writable :: from_reg (r)) }
};
}
