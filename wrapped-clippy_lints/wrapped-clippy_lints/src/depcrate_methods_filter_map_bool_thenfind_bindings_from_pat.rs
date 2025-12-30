// Generated macro for find_bindings_from_pat (function)
macro_rules! Depcrate_methods_filter_map_bool_thenfind_bindings_from_pat {
() => {
// Module: crate::methods::filter_map_bool_then
// Provides: {"find_bindings_from_pat"}
// Dependencies: {}
# [doc = " Returns a set of all bindings found in the given pattern."] fn find_bindings_from_pat (pat : & Pat < '_ >) -> FxHashSet < HirId > { let mut bindings = FxHashSet :: default () ; pat . walk (| p | { if let rustc_hir :: PatKind :: Binding (_ , hir_id , _ , _) = p . kind { bindings . insert (hir_id) ; } true }) ; bindings }
};
}
