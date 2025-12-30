// Generated macro for check_sized_if_body (function)
macro_rules! Depcrate_check_wfcheckcheck_sized_if_body {
() => {
// Module: crate::check::wfcheck
// Provides: {"check_sized_if_body"}
// Dependencies: {}
fn check_sized_if_body < 'tcx > (wfcx : & WfCheckingCtxt < '_ , 'tcx > , def_id : LocalDefId , ty : Ty < 'tcx > , maybe_span : Option < Span > , code : ObligationCauseCode < 'tcx > ,) { let tcx = wfcx . tcx () ; if let Some (body) = tcx . hir_maybe_body_owned_by (def_id) { let span = maybe_span . unwrap_or (body . value . span) ; wfcx . register_bound (ObligationCause :: new (span , def_id , code) , wfcx . param_env , ty , tcx . require_lang_item (LangItem :: Sized , span) ,) ; } }
};
}
