// Generated macro for impl_679 (impl)
macro_rules! Depcrate_ir_entitiesimpl_679 {
() => {
// Module: crate::ir::entities
// Provides: {"impl_679"}
// Dependencies: {}
impl fmt :: Display for AnyEntity { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match * self { Self :: Function => write ! (f , "function") , Self :: Block (r) => r . fmt (f) , Self :: Inst (r) => r . fmt (f) , Self :: Value (r) => r . fmt (f) , Self :: StackSlot (r) => r . fmt (f) , Self :: DynamicStackSlot (r) => r . fmt (f) , Self :: DynamicType (r) => r . fmt (f) , Self :: GlobalValue (r) => r . fmt (f) , Self :: MemoryType (r) => r . fmt (f) , Self :: JumpTable (r) => r . fmt (f) , Self :: Constant (r) => r . fmt (f) , Self :: FuncRef (r) => r . fmt (f) , Self :: SigRef (r) => r . fmt (f) , Self :: StackLimit => write ! (f , "stack_limit") , } } }
};
}
