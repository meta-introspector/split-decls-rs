// Generated macro for mutation_detected (function)
macro_rules! Depcrate_itermutation_detected {
() => {
// Module: crate::iter
// Provides: {"mutation_detected"}
// Dependencies: {}
# [cfg (not (feature = "unstable-mutation-return-null"))] # [cfg_attr (debug_assertions , track_caller)] fn mutation_detected () -> ! { panic ! ("mutation detected during enumeration") ; }
};
}
