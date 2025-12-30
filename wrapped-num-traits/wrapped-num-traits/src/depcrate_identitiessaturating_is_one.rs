// Generated macro for saturating_is_one (function)
macro_rules! Depcrate_identitiessaturating_is_one {
() => {
// Module: crate::identities
// Provides: {"saturating_is_one"}
// Dependencies: {}
# [test] # [cfg (has_num_saturating)] fn saturating_is_one () { fn require_one < T : One > (_ : & T) { } require_one (& Saturating (42)) ; }
};
}
