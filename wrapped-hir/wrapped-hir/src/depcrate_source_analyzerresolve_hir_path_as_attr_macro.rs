// Generated macro for resolve_hir_path_as_attr_macro (function)
macro_rules! Depcrate_source_analyzerresolve_hir_path_as_attr_macro {
() => {
// Module: crate::source_analyzer
// Provides: {"resolve_hir_path_as_attr_macro"}
// Dependencies: {}
# [inline] pub (crate) fn resolve_hir_path_as_attr_macro (db : & dyn HirDatabase , resolver : & Resolver < '_ > , path : & Path ,) -> Option < Macro > { resolver . resolve_path_as_macro (db , path . mod_path () ? , Some (MacroSubNs :: Attr)) . map (| (it , _) | it) . map (Into :: into) }
};
}
