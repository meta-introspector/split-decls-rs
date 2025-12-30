// Generated macro for impl_122 (impl)
macro_rules! Depcrate_absolute_pathsimpl_122 {
() => {
// Module: crate::absolute_paths
// Provides: {"impl_122"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for AbsolutePaths { fn check_path (& mut self , cx : & LateContext < 'tcx > , path : & Path < 'tcx > , hir_id : HirId) { let segments = match path . segments { [] | [_] => return , [rest @ .. , _] if let [.. , s] = rest && matches ! (s . res , Res :: Def (DefKind :: Enum | DefKind :: Trait | DefKind :: TraitAlias , _)) => { rest } , path => path , } ; if let [s1 , s2 , ..] = segments && let has_root = s1 . ident . name == kw :: PathRoot && let first = if has_root { s2 } else { s1 } && let len = segments . len () - usize :: from (has_root) && len as u64 > self . absolute_paths_max_segments && let crate_name = if let Res :: Def (DefKind :: Mod , DefId { index , .. }) = first . res && index == CRATE_DEF_INDEX { first . ident . name } else if first . ident . name == kw :: Crate || has_root { kw :: Crate } else { return ; } && ! path . span . from_expansion () && let node = cx . tcx . hir_node (hir_id) && ! matches ! (node , Node :: Item (item) if matches ! (item . kind , ItemKind :: Use (..))) && ! self . absolute_paths_allowed_crates . contains (& crate_name) && ! is_from_proc_macro (cx , path) { span_lint (cx , ABSOLUTE_PATHS , path . span , "consider bringing this path into scope with the `use` keyword" ,) ; } } }
};
}
