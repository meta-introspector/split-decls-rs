// Generated macro for impl_57 (impl)
macro_rules! Depcrate_navigation_targetimpl_57 {
() => {
// Module: crate::navigation_target
// Provides: {"impl_57"}
// Dependencies: {}
impl TryToNav for hir :: AssocItem { fn try_to_nav (& self , sema : & Semantics < '_ , RootDatabase > ,) -> Option < UpmappingResult < NavigationTarget > > { match self { AssocItem :: Function (it) => it . try_to_nav (sema) , AssocItem :: Const (it) => it . try_to_nav (sema) , AssocItem :: TypeAlias (it) => it . try_to_nav (sema) , } } }
};
}
