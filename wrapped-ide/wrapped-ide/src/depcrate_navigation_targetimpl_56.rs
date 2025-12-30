// Generated macro for impl_56 (impl)
macro_rules! Depcrate_navigation_targetimpl_56 {
() => {
// Module: crate::navigation_target
// Provides: {"impl_56"}
// Dependencies: {}
impl TryToNav for hir :: Adt { fn try_to_nav (& self , sema : & Semantics < '_ , RootDatabase > ,) -> Option < UpmappingResult < NavigationTarget > > { match self { hir :: Adt :: Struct (it) => it . try_to_nav (sema) , hir :: Adt :: Union (it) => it . try_to_nav (sema) , hir :: Adt :: Enum (it) => it . try_to_nav (sema) , } } }
};
}
