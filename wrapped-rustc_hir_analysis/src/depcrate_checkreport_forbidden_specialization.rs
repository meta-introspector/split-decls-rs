// Generated macro for report_forbidden_specialization (function)
macro_rules! Depcrate_checkreport_forbidden_specialization {
() => {
// Module: crate::check
// Provides: {"report_forbidden_specialization"}
// Dependencies: {}
fn report_forbidden_specialization (tcx : TyCtxt < '_ > , impl_item : DefId , parent_impl : DefId) { let span = tcx . def_span (impl_item) ; let ident = tcx . item_ident (impl_item) ; let err = match tcx . span_of_impl (parent_impl) { Ok (sp) => errors :: ImplNotMarkedDefault :: Ok { span , ident , ok_label : sp } , Err (cname) => errors :: ImplNotMarkedDefault :: Err { span , ident , cname } , } ; tcx . dcx () . emit_err (err) ; }
};
}
