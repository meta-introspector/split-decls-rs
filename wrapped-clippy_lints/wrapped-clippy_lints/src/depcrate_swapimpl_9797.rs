// Generated macro for impl_9797 (impl)
macro_rules! Depcrate_swapimpl_9797 {
() => {
// Module: crate::swap
// Provides: {"impl_9797"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for Swap { fn check_block (& mut self , cx : & LateContext < 'tcx > , block : & 'tcx Block < '_ >) { check_manual_swap (cx , block) ; check_suspicious_swap (cx , block) ; check_xor_swap (cx , block) ; } }
};
}
