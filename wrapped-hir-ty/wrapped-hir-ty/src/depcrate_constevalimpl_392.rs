// Generated macro for impl_392 (impl)
macro_rules! Depcrate_constevalimpl_392 {
() => {
// Module: crate::consteval
// Provides: {"impl_392"}
// Dependencies: {}
impl ConstExt for Const { fn is_unknown (& self) -> bool { match self . data (Interner) . value { chalk_ir :: ConstValue :: Concrete (chalk_ir :: ConcreteConst { interned : ConstScalar :: Unknown , }) => true , chalk_ir :: ConstValue :: Concrete (..) => false , _ => { tracing :: error ! ("is_unknown was called on a non-concrete constant value! {:?}" , self) ; true } } } }
};
}
