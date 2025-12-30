// Generated macro for impl_205 (impl)
macro_rules! Depcrate_hirimpl_205 {
() => {
// Module: crate::hir
// Provides: {"impl_205"}
// Dependencies: {}
impl LifetimeKind { fn is_elided (& self) -> bool { match self { LifetimeKind :: ImplicitObjectLifetimeDefault | LifetimeKind :: Infer => true , LifetimeKind :: Error | LifetimeKind :: Param (..) | LifetimeKind :: Static => false , } } }
};
}
