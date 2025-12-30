// Generated macro for check_sig (function)
macro_rules! Depcrate_eta_reductioncheck_sig {
() => {
// Module: crate::eta_reduction
// Provides: {"check_sig"}
// Dependencies: {}
fn check_sig < 'tcx > (closure_sig : FnSig < 'tcx > , call_sig : FnSig < 'tcx >) -> bool { call_sig . safety . is_safe () && ! has_late_bound_to_non_late_bound_regions (closure_sig , call_sig) }
};
}
