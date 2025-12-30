// Generated macro for adt_async_destructor (function)
macro_rules! Depcrate_checkadt_async_destructor {
() => {
// Module: crate::check
// Provides: {"adt_async_destructor"}
// Dependencies: {}
fn adt_async_destructor (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> Option < ty :: AsyncDestructor > { tcx . calculate_async_dtor (def_id , always_applicable :: check_drop_impl) }
};
}
