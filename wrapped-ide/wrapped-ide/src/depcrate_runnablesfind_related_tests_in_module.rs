// Generated macro for find_related_tests_in_module (function)
macro_rules! Depcrate_runnablesfind_related_tests_in_module {
() => {
// Module: crate::runnables
// Provides: {"find_related_tests_in_module"}
// Dependencies: {}
fn find_related_tests_in_module (sema : & Semantics < '_ , RootDatabase > , syntax : & SyntaxNode , fn_def : & ast :: Fn , parent_module : & hir :: Module , tests : & mut FxIndexSet < Runnable > ,) { let fn_name = match fn_def . name () { Some (it) => it , _ => return , } ; let mod_source = parent_module . definition_source_range (sema . db) ; let file_id = mod_source . file_id . original_file (sema . db) ; let mod_scope = SearchScope :: file_range (hir :: FileRange { file_id , range : mod_source . value }) ; let fn_pos = FilePosition { file_id : file_id . file_id (sema . db) , offset : fn_name . syntax () . text_range () . start () , } ; find_related_tests (sema , syntax , fn_pos , Some (mod_scope) , tests) }
};
}
