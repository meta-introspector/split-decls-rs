// Generated macro for check_well_formed (function)
macro_rules! Depcrate_check_wfcheckcheck_well_formed {
() => {
// Module: crate::check::wfcheck
// Provides: {"check_well_formed"}
// Dependencies: {}
pub (super) fn check_well_formed (tcx : TyCtxt < '_ > , def_id : LocalDefId ,) -> Result < () , ErrorGuaranteed > { let mut res = crate :: check :: check :: check_item_type (tcx , def_id) ; for param in & tcx . generics_of (def_id) . own_params { res = res . and (check_param_wf (tcx , param)) ; } res }
};
}
