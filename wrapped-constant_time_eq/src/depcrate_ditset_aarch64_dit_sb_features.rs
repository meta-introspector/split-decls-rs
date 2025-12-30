// Generated macro for set_aarch64_dit_sb_features (function)
macro_rules! Depcrate_ditset_aarch64_dit_sb_features {
() => {
// Module: crate::dit
// Provides: {"set_aarch64_dit_sb_features"}
// Dependencies: {}
# [doc = " Overrides the runtime detection of `FEAT_DIT` and `FEAT_SB`."] # [doc = ""] # [doc = " This must be called before other threads are created, and before"] # [doc = " any other code in this `constant_time_eq` crate is called."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Either parameter must not be set to true if the corresponding feature"] # [doc = " is not implemented."] pub unsafe fn set_aarch64_dit_sb_features (_dit : bool , _sb : bool) { # [cfg (not (all (target_feature = "dit" , target_feature = "sb")))] unsafe { detect :: set_aarch64_dit_sb_features (_dit , _sb) ; } }
};
}
