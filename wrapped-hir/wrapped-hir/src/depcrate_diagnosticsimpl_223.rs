// Generated macro for impl_223 (impl)
macro_rules! Depcrate_diagnosticsimpl_223 {
() => {
// Module: crate::diagnostics
// Provides: {"impl_223"}
// Dependencies: {}
impl GenericArgKind { fn from_id (id : GenericParamId) -> Self { match id { GenericParamId :: TypeParamId (_) => GenericArgKind :: Type , GenericParamId :: ConstParamId (_) => GenericArgKind :: Const , GenericParamId :: LifetimeParamId (_) => GenericArgKind :: Lifetime , } } }
};
}
