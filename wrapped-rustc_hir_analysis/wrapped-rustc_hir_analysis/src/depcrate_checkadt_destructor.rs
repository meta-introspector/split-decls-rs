// Generated macro for adt_destructor (function)
macro_rules! Depcrate_checkadt_destructor {
() => {
// Module: crate::check
// Provides: {"adt_destructor"}
// Dependencies: {}
fn adt_destructor (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> Option < ty :: Destructor > { let dtor = tcx . calculate_dtor (def_id , always_applicable :: check_drop_impl) ; if dtor . is_none () && tcx . features () . async_drop () { if let Some (async_dtor) = adt_async_destructor (tcx , def_id) { let span = tcx . def_span (async_dtor . impl_did) ; tcx . dcx () . emit_err (errors :: AsyncDropWithoutSyncDrop { span }) ; } } dtor }
};
}
