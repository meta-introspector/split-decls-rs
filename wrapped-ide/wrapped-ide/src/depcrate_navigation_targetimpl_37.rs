// Generated macro for impl_37 (impl)
macro_rules! Depcrate_navigation_targetimpl_37 {
() => {
// Module: crate::navigation_target
// Provides: {"impl_37"}
// Dependencies: {}
impl TryToNav for hir :: ModuleDef { fn try_to_nav (& self , sema : & Semantics < '_ , RootDatabase > ,) -> Option < UpmappingResult < NavigationTarget > > { match self { hir :: ModuleDef :: Module (it) => Some (it . to_nav (sema . db)) , hir :: ModuleDef :: Function (it) => it . try_to_nav (sema) , hir :: ModuleDef :: Adt (it) => it . try_to_nav (sema) , hir :: ModuleDef :: Variant (it) => it . try_to_nav (sema) , hir :: ModuleDef :: Const (it) => it . try_to_nav (sema) , hir :: ModuleDef :: Static (it) => it . try_to_nav (sema) , hir :: ModuleDef :: Trait (it) => it . try_to_nav (sema) , hir :: ModuleDef :: TypeAlias (it) => it . try_to_nav (sema) , hir :: ModuleDef :: Macro (it) => it . try_to_nav (sema) , hir :: ModuleDef :: BuiltinType (_) => None , } } }
};
}
