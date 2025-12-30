// Generated macro for cfg_default (macro)
macro_rules! Depcrate_utilscfg_default {
() => {
// Module: crate::utils
// Provides: {"cfg_default"}
// Dependencies: {}
# [doc = " Declares default items."] # [allow (unused_macros)] # [doc (hidden)] macro_rules ! cfg_default { ($ ($ item : item) *) => { $ (# [cfg (feature = "default")] $ item) * } }
};
}
