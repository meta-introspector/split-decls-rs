// Generated macro for get_aarch64_dit_sb_features (function)
macro_rules! Depcrate_ditget_aarch64_dit_sb_features {
() => {
// Module: crate::dit
// Provides: {"get_aarch64_dit_sb_features"}
// Dependencies: {}
# [doc = " Determines whether `FEAT_DIT` and `FEAT_SB` are known to be implemented."] # [cfg (all (target_feature = "dit" , target_feature = "sb"))] # [inline (always)] fn get_aarch64_dit_sb_features () -> Features { Features :: DitSb }
};
}
