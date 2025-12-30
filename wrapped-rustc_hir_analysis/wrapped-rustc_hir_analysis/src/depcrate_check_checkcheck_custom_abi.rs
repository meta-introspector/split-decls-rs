// Generated macro for check_custom_abi (function)
macro_rules! Depcrate_check_checkcheck_custom_abi {
() => {
// Module: crate::check::check
// Provides: {"check_custom_abi"}
// Dependencies: {}
pub fn check_custom_abi (tcx : TyCtxt < '_ > , def_id : LocalDefId , fn_sig : FnSig < '_ > , fn_sig_span : Span) { if fn_sig . abi == ExternAbi :: Custom { if ! find_attr ! (tcx . get_all_attrs (def_id) , AttributeKind :: Naked (_)) { tcx . dcx () . emit_err (crate :: errors :: AbiCustomClothedFunction { span : fn_sig_span , naked_span : tcx . def_span (def_id) . shrink_to_lo () , }) ; } } }
};
}
