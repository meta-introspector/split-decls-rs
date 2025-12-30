// Generated macro for impl_592 (impl)
macro_rules! Depcrate_ir_dfgimpl_592 {
() => {
// Module: crate::ir::dfg
// Provides: {"impl_592"}
// Dependencies: {}
# [doc = " Allow immutable access to instructions via indexing."] impl Index < Inst > for Insts { type Output = InstructionData ; fn index (& self , inst : Inst) -> & InstructionData { self . 0 . index (inst) } }
};
}
