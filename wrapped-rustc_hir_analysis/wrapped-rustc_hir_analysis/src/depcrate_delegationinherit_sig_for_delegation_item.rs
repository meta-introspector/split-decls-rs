// Generated macro for inherit_sig_for_delegation_item (function)
macro_rules! Depcrate_delegationinherit_sig_for_delegation_item {
() => {
// Module: crate::delegation
// Provides: {"inherit_sig_for_delegation_item"}
// Dependencies: {}
pub (crate) fn inherit_sig_for_delegation_item < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : LocalDefId ,) -> & 'tcx [Ty < 'tcx >] { let sig_id = tcx . hir_opt_delegation_sig_id (def_id) . unwrap () ; let caller_sig = tcx . fn_sig (sig_id) ; if let Err (err) = check_constraints (tcx , def_id , sig_id) { let sig_len = caller_sig . instantiate_identity () . skip_binder () . inputs () . len () + 1 ; let err_type = Ty :: new_error (tcx , err) ; return tcx . arena . alloc_from_iter ((0 .. sig_len) . map (| _ | err_type)) ; } let args = create_generic_args (tcx , def_id , sig_id) ; let sig = caller_sig . instantiate (tcx , args) . skip_binder () ; let sig_iter = sig . inputs () . iter () . cloned () . chain (std :: iter :: once (sig . output ())) ; tcx . arena . alloc_from_iter (sig_iter) }
};
}
