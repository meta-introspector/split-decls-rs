// Generated macro for scope_def_to_name (function)
macro_rules! Depcrate_renderscope_def_to_name {
() => {
// Module: crate::render
// Provides: {"scope_def_to_name"}
// Dependencies: {}
fn scope_def_to_name (resolution : ScopeDef , ctx : & RenderContext < '_ > , import_edit : & LocatedImport ,) -> Option < hir :: Name > { Some (match resolution { ScopeDef :: ModuleDef (hir :: ModuleDef :: Function (f)) => f . name (ctx . completion . db) , ScopeDef :: ModuleDef (hir :: ModuleDef :: Const (c)) => c . name (ctx . completion . db) ? , ScopeDef :: ModuleDef (hir :: ModuleDef :: TypeAlias (t)) => t . name (ctx . completion . db) , _ => item_name (ctx . db () , import_edit . original_item) ? , }) }
};
}
