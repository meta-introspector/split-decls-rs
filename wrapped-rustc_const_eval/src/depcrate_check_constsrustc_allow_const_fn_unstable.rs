// Generated macro for rustc_allow_const_fn_unstable (function)
macro_rules! Depcrate_check_constsrustc_allow_const_fn_unstable {
() => {
// Module: crate::check_consts
// Provides: {"rustc_allow_const_fn_unstable"}
// Dependencies: {}
pub fn rustc_allow_const_fn_unstable (tcx : TyCtxt < '_ > , def_id : LocalDefId , feature_gate : Symbol ,) -> bool { let attrs = tcx . hir_attrs (tcx . local_def_id_to_hir_id (def_id)) ; find_attr ! (attrs , AttributeKind :: AllowConstFnUnstable (syms , _) if syms . contains (& feature_gate)) }
};
}
