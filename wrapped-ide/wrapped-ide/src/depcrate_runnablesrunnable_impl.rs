// Generated macro for runnable_impl (function)
macro_rules! Depcrate_runnablesrunnable_impl {
() => {
// Module: crate::runnables
// Provides: {"runnable_impl"}
// Dependencies: {}
pub (crate) fn runnable_impl (sema : & Semantics < '_ , RootDatabase > , def : & hir :: Impl ,) -> Option < Runnable > { let display_target = def . module (sema . db) . krate () . to_display_target (sema . db) ; let edition = display_target . edition ; let attrs = def . attrs (sema . db) ; if ! has_runnable_doc_test (& attrs) { return None ; } let cfg = attrs . cfg () ; let nav = def . try_to_nav (sema) ? . call_site () ; let ty = def . self_ty (sema . db) ; let adt_name = ty . as_adt () ? . name (sema . db) ; let mut ty_args = ty . generic_parameters (sema . db , display_target) . peekable () ; let params = hir :: attach_db (sema . db , | | { if ty_args . peek () . is_some () { format ! ("<{}>" , ty_args . format_with ("," , | ty , cb | cb (& ty))) } else { String :: new () } }) ; let mut test_id = format ! ("{}{params}" , adt_name . display (sema . db , edition)) ; test_id . retain (| c | c != ' ') ; let test_id = TestId :: Path (test_id) ; let impl_source = sema . source (* def) ? ; let impl_syntax = impl_source . syntax () ; let file_range = impl_syntax . original_file_range_with_macro_call_input (sema . db) ; let update_test = UpdateTest :: find_snapshot_macro (sema , file_range) ; Some (Runnable { use_name_in_title : false , nav , kind : RunnableKind :: DocTest { test_id } , cfg , update_test , }) }
};
}
