// Generated macro for check_item_fn (function)
macro_rules! Depcrate_check_wfcheckcheck_item_fn {
() => {
// Module: crate::check::wfcheck
// Provides: {"check_item_fn"}
// Dependencies: {}
fn check_item_fn (tcx : TyCtxt < '_ > , def_id : LocalDefId , decl : & hir :: FnDecl < '_ > ,) -> Result < () , ErrorGuaranteed > { enter_wf_checking_ctxt (tcx , def_id , | wfcx | { let sig = tcx . fn_sig (def_id) . instantiate_identity () ; check_fn_or_method (wfcx , sig , decl , def_id) ; Ok (()) }) }
};
}
