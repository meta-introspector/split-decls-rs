// Generated macro for impl_593 (impl)
macro_rules! Depcrate_ir_dfgimpl_593 {
() => {
// Module: crate::ir::dfg
// Provides: {"impl_593"}
// Dependencies: {}
# [doc = " Allow mutable access to instructions via indexing."] impl IndexMut < Inst > for Insts { fn index_mut (& mut self , inst : Inst) -> & mut InstructionData { self . 0 . index_mut (inst) } }
};
}
