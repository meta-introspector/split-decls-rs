// Generated macro for impl_355 (impl)
macro_rules! Depcrate_hirimpl_355 {
() => {
// Module: crate::hir
// Provides: {"impl_355"}
// Dependencies: {}
impl < 'hir > AssocItemConstraintKind < 'hir > { pub fn descr (& self) -> & 'static str { match self { AssocItemConstraintKind :: Equality { .. } => "binding" , AssocItemConstraintKind :: Bound { .. } => "constraint" , } } }
};
}
