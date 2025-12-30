// Generated macro for runnable_fn (function)
macro_rules! Depcrate_runnablesrunnable_fn {
() => {
// Module: crate::runnables
// Provides: {"runnable_fn"}
// Dependencies: {}
pub (crate) fn runnable_fn (sema : & Semantics < '_ , RootDatabase > , def : hir :: Function ,) -> Option < Runnable > { let edition = def . krate (sema . db) . edition (sema . db) ; let under_cfg_test = has_cfg_test (def . module (sema . db) . attrs (sema . db)) ; let kind = if ! under_cfg_test && def . is_main (sema . db) { RunnableKind :: Bin } else { let test_id = | | { let canonical_path = { let def : hir :: ModuleDef = def . into () ; def . canonical_path (sema . db , edition) } ; canonical_path . map (TestId :: Path) . unwrap_or (TestId :: Name (def . name (sema . db) . display_no_db (edition) . to_smolstr ())) } ; if def . is_test (sema . db) { let attr = TestAttr :: from_fn (sema . db , def) ; RunnableKind :: Test { test_id : test_id () , attr } } else if def . is_bench (sema . db) { RunnableKind :: Bench { test_id : test_id () } } else { return None ; } } ; let fn_source = sema . source (def) ? ; let nav = NavigationTarget :: from_named (sema . db , fn_source . as_ref () . map (| it | it as & dyn ast :: HasName) , SymbolKind :: Function ,) . call_site () ; let file_range = fn_source . syntax () . original_file_range_with_macro_call_input (sema . db) ; let update_test = UpdateTest :: find_snapshot_macro (sema , file_range) ; let cfg = def . attrs (sema . db) . cfg () ; Some (Runnable { use_name_in_title : false , nav , kind , cfg , update_test }) }
};
}
