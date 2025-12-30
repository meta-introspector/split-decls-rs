// Generated macro for tests (module)
macro_rules! Depcrate_dittests {
() => {
// Module: crate::dit
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { extern crate std ; use super :: { rsr64_dit , with_dit } ; use std :: arch :: is_aarch64_feature_detected ; # [test] fn dit_is_restored_after_with_dit () { if is_aarch64_feature_detected ! ("dit") { unsafe { let saved = rsr64_dit () ; with_dit (| | assert_ne ! (rsr64_dit () , 0)) ; assert_eq ! (rsr64_dit () , saved) ; } } } }
};
}
