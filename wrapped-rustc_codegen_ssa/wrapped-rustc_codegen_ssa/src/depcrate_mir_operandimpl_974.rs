// Generated macro for impl_974 (impl)
macro_rules! Depcrate_mir_operandimpl_974 {
() => {
// Module: crate::mir::operand
// Provides: {"impl_974"}
// Dependencies: {}
impl < V : CodegenObject > fmt :: Debug for OperandRef < '_ , V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "OperandRef({:?} @ {:?})" , self . val , self . layout) } }
};
}
