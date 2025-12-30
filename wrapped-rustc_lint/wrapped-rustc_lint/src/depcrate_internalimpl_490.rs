// Generated macro for impl_490 (impl)
macro_rules! Depcrate_internalimpl_490 {
() => {
// Module: crate::internal
// Provides: {"impl_490"}
// Dependencies: {}
impl LateLintPass < '_ > for DefaultHashTypes { fn check_path (& mut self , cx : & LateContext < '_ > , path : & hir :: Path < '_ > , hir_id : HirId) { let Res :: Def (rustc_hir :: def :: DefKind :: Struct , def_id) = path . res else { return } ; if matches ! (cx . tcx . hir_node (hir_id) , hir :: Node :: Item (hir :: Item { kind : hir :: ItemKind :: Use (..) , .. })) { return ; } let preferred = match cx . tcx . get_diagnostic_name (def_id) { Some (sym :: HashMap) => "FxHashMap" , Some (sym :: HashSet) => "FxHashSet" , _ => return , } ; cx . emit_span_lint (DEFAULT_HASH_TYPES , path . span , DefaultHashTypesDiag { preferred , used : cx . tcx . item_name (def_id) } ,) ; } }
};
}
