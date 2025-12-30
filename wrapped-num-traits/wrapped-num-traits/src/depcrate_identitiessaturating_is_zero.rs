// Generated macro for saturating_is_zero (function)
macro_rules! Depcrate_identitiessaturating_is_zero {
() => {
// Module: crate::identities
// Provides: {"saturating_is_zero"}
// Dependencies: {}
# [test] # [cfg (has_num_saturating)] fn saturating_is_zero () { fn require_zero < T : Zero > (_ : & T) { } require_zero (& Saturating (42)) ; }
};
}
