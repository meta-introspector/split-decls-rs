// Generated macro for resolve_target_trait (function)
macro_rules! Depcrate_traitsresolve_target_trait {
() => {
// Module: crate::traits
// Provides: {"resolve_target_trait"}
// Dependencies: {}
# [doc = " Given the `impl` block, attempts to find the trait this `impl` corresponds to."] pub fn resolve_target_trait (sema : & Semantics < '_ , RootDatabase > , impl_def : & ast :: Impl ,) -> Option < hir :: Trait > { let ast_path = impl_def . trait_ () . map (| it | it . syntax () . clone ()) . and_then (ast :: PathType :: cast) ? . path () ? ; match sema . resolve_path (& ast_path) { Some (hir :: PathResolution :: Def (hir :: ModuleDef :: Trait (def))) => Some (def) , _ => None , } }
};
}
