// Generated macro for impl_58 (impl)
macro_rules! Depcrate_navigation_targetimpl_58 {
() => {
// Module: crate::navigation_target
// Provides: {"impl_58"}
// Dependencies: {}
impl TryToNav for hir :: GenericParam { fn try_to_nav (& self , sema : & Semantics < '_ , RootDatabase > ,) -> Option < UpmappingResult < NavigationTarget > > { match self { hir :: GenericParam :: TypeParam (it) => it . try_to_nav (sema) , hir :: GenericParam :: ConstParam (it) => it . try_to_nav (sema) , hir :: GenericParam :: LifetimeParam (it) => it . try_to_nav (sema) , } } }
};
}
