// Generated macro for impl_94 (impl)
macro_rules! Depcrate_builtinimpl_94 {
() => {
// Module: crate::builtin
// Provides: {"impl_94"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for MissingDebugImplementations { fn check_item (& mut self , cx : & LateContext < '_ > , item : & hir :: Item < '_ >) { if ! cx . effective_visibilities . is_reachable (item . owner_id . def_id) { return ; } match item . kind { hir :: ItemKind :: Struct (..) | hir :: ItemKind :: Union (..) | hir :: ItemKind :: Enum (..) => { } _ => return , } let LevelAndSource { level , .. } = cx . tcx . lint_level_at_node (MISSING_DEBUG_IMPLEMENTATIONS , item . hir_id ()) ; if level == Level :: Allow { return ; } let Some (debug) = cx . tcx . get_diagnostic_item (sym :: Debug) else { return } ; let has_impl = cx . tcx . non_blanket_impls_for_ty (debug , cx . tcx . type_of (item . owner_id) . instantiate_identity ()) . next () . is_some () ; if ! has_impl { cx . emit_span_lint (MISSING_DEBUG_IMPLEMENTATIONS , item . span , BuiltinMissingDebugImpl { tcx : cx . tcx , def_id : debug } ,) ; } } }
};
}
