// Generated macro for check_opaque_for_cycles (function)
macro_rules! Depcrate_check_checkcheck_opaque_for_cycles {
() => {
// Module: crate::check::check
// Provides: {"check_opaque_for_cycles"}
// Dependencies: {}
# [doc = " Checks that an opaque type does not contain cycles."] pub (super) fn check_opaque_for_cycles < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : LocalDefId ,) -> Result < () , ErrorGuaranteed > { let args = GenericArgs :: identity_for_item (tcx , def_id) ; if tcx . try_expand_impl_trait_type (def_id . to_def_id () , args) . is_err () { let reported = opaque_type_cycle_error (tcx , def_id) ; return Err (reported) ; } Ok (()) }
};
}
