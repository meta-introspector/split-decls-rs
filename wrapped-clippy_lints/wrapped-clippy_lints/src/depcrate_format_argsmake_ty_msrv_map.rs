// Generated macro for make_ty_msrv_map (function)
macro_rules! Depcrate_format_argsmake_ty_msrv_map {
() => {
// Module: crate::format_args
// Provides: {"make_ty_msrv_map"}
// Dependencies: {}
fn make_ty_msrv_map (tcx : TyCtxt < '_ >) -> FxHashMap < Ty < '_ > , Option < RustcVersion > > { [(sym :: OsStr , Some (msrvs :: OS_STR_DISPLAY)) , (sym :: Path , None)] . into_iter () . filter_map (| (name , feature) | { tcx . get_diagnostic_item (name) . map (| def_id | { let ty = Ty :: new_adt (tcx , tcx . adt_def (def_id) , List :: empty ()) ; (ty , feature) }) }) . collect () }
};
}
