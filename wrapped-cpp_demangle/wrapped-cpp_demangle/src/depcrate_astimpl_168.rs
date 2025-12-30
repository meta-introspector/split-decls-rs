// Generated macro for impl_168 (impl)
macro_rules! Depcrate_astimpl_168 {
() => {
// Module: crate::ast
// Provides: {"impl_168"}
// Dependencies: {}
impl CtorDtorName { fn inheriting_mut (& mut self) -> & mut Option < TypeHandle > { match self { CtorDtorName :: CompleteConstructor (ref mut inheriting) | CtorDtorName :: BaseConstructor (ref mut inheriting) | CtorDtorName :: CompleteAllocatingConstructor (ref mut inheriting) | CtorDtorName :: MaybeInChargeConstructor (ref mut inheriting) => inheriting , CtorDtorName :: DeletingDestructor | CtorDtorName :: CompleteDestructor | CtorDtorName :: BaseDestructor | CtorDtorName :: MaybeInChargeDestructor => unreachable ! () , } } }
};
}
