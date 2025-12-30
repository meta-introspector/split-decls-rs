// Generated macro for cfg_std (macro)
macro_rules! Depcrate_utilscfg_std {
() => {
// Module: crate::utils
// Provides: {"cfg_std"}
// Dependencies: {}
# [doc = " Declares std items."] # [allow (unused_macros)] # [doc (hidden)] macro_rules ! cfg_std { ($ ($ item : item) *) => { $ (# [cfg (feature = "std")] $ item) * } }
};
}
