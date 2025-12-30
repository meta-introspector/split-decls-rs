// Generated macro for runnable_mod (function)
macro_rules! Depcrate_runnablesrunnable_mod {
() => {
// Module: crate::runnables
// Provides: {"runnable_mod"}
// Dependencies: {}
pub (crate) fn runnable_mod (sema : & Semantics < '_ , RootDatabase > , def : hir :: Module ,) -> Option < Runnable > { if ! has_test_function_or_multiple_test_submodules (sema , & def , has_cfg_test (def . attrs (sema . db))) { return None ; } let path = def . path_to_root (sema . db) . into_iter () . rev () . filter_map (| module | { module . name (sema . db) . map (| mod_name | { mod_name . display (sema . db , module . krate () . edition (sema . db)) . to_string () }) }) . join ("::") ; let attrs = def . attrs (sema . db) ; let cfg = attrs . cfg () ; let nav = NavigationTarget :: from_module_to_decl (sema . db , def) . call_site () ; let module_source = sema . module_definition_node (def) ; let module_syntax = module_source . file_syntax (sema . db) ; let file_range = hir :: FileRange { file_id : module_source . file_id . original_file (sema . db) , range : module_syntax . text_range () , } ; let update_test = UpdateTest :: find_snapshot_macro (sema , file_range) ; Some (Runnable { use_name_in_title : false , nav , kind : RunnableKind :: TestMod { path } , cfg , update_test , }) }
};
}
