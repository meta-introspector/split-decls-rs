// Generated macro for check_abi (function)
macro_rules! Depcrate_check_checkcheck_abi {
() => {
// Module: crate::check::check
// Provides: {"check_abi"}
// Dependencies: {}
pub fn check_abi (tcx : TyCtxt < '_ > , hir_id : hir :: HirId , span : Span , abi : ExternAbi) { match AbiMap :: from_target (& tcx . sess . target) . canonize_abi (abi , false) { AbiMapping :: Direct (..) => () , AbiMapping :: Invalid => { tcx . dcx () . span_delayed_bug (span , format ! ("{abi} should be rejected in ast_lowering")) ; } AbiMapping :: Deprecated (..) => { tcx . node_span_lint (UNSUPPORTED_CALLING_CONVENTIONS , hir_id , span , | lint | { lint . primary_message (format ! ("{abi} is not a supported ABI for the current target")) ; add_abi_diag_help (abi , lint) ; }) ; } } }
};
}
