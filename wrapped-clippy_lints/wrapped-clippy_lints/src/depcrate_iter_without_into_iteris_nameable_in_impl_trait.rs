// Generated macro for is_nameable_in_impl_trait (function)
macro_rules! Depcrate_iter_without_into_iteris_nameable_in_impl_trait {
() => {
// Module: crate::iter_without_into_iter
// Provides: {"is_nameable_in_impl_trait"}
// Dependencies: {}
# [doc = " Checks if a given type is nameable in a trait (impl)."] # [doc = " RPIT is stable, but impl Trait in traits is not (yet), so when we have"] # [doc = " a function such as `fn iter(&self) -> impl IntoIterator`, we can't"] # [doc = " suggest `type IntoIter = impl IntoIterator`."] fn is_nameable_in_impl_trait (ty : & rustc_hir :: Ty < '_ >) -> bool { ! matches ! (ty . kind , TyKind :: OpaqueDef (..)) }
};
}
