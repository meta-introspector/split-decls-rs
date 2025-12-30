// Generated macro for impl_63 (impl)
macro_rules! Depcrate_navigation_targetimpl_63 {
() => {
// Module: crate::navigation_target
// Provides: {"impl_63"}
// Dependencies: {}
impl TryToNav for hir :: TypeOrConstParam { fn try_to_nav (& self , sema : & Semantics < '_ , RootDatabase > ,) -> Option < UpmappingResult < NavigationTarget > > { self . split (sema . db) . try_to_nav (sema) } }
};
}
