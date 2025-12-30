// Generated macro for retpoline_features_by_flags (function)
macro_rules! Depcrate_target_featuresretpoline_features_by_flags {
() => {
// Module: crate::target_features
// Provides: {"retpoline_features_by_flags"}
// Dependencies: {}
# [doc = " Computes the backend target features to be added to account for retpoline flags."] # [doc = " Used by both LLVM and GCC since their target features are, conveniently, the same."] pub fn retpoline_features_by_flags (sess : & Session , features : & mut Vec < String >) { let unstable_opts = & sess . opts . unstable_opts ; if unstable_opts . retpoline && ! unstable_opts . retpoline_external_thunk { features . push ("+retpoline-indirect-branches" . into ()) ; features . push ("+retpoline-indirect-calls" . into ()) ; } if unstable_opts . retpoline_external_thunk { features . push ("+retpoline-external-thunk" . into ()) ; features . push ("+retpoline-indirect-branches" . into ()) ; features . push ("+retpoline-indirect-calls" . into ()) ; } }
};
}
