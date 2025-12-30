// Generated macro for enclosing_mir (function)
macro_rules! Depcrate_mirenclosing_mir {
() => {
// Module: crate::mir
// Provides: {"enclosing_mir"}
// Dependencies: {}
# [doc = " Returns the `mir::Body` containing the node associated with `hir_id`."] # [expect (clippy :: module_name_repetitions)] pub fn enclosing_mir (tcx : TyCtxt < '_ > , hir_id : HirId) -> Option < & Body < '_ > > { let body_owner_local_def_id = tcx . hir_enclosing_body_owner (hir_id) ; if tcx . hir_body_owner_kind (body_owner_local_def_id) . is_fn_or_closure () { Some (tcx . optimized_mir (body_owner_local_def_id . to_def_id ())) } else { None } }
};
}
