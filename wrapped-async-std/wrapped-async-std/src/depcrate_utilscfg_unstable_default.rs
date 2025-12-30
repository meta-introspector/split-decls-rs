// Generated macro for cfg_unstable_default (macro)
macro_rules! Depcrate_utilscfg_unstable_default {
() => {
// Module: crate::utils
// Provides: {"cfg_unstable_default"}
// Dependencies: {}
# [doc = " Declares unstable and default items."] # [doc (hidden)] macro_rules ! cfg_unstable_default { ($ ($ item : item) *) => { $ (# [cfg (all (feature = "default" , feature = "unstable"))] # [cfg_attr (feature = "docs" , doc (cfg (unstable)))] $ item) * } }
};
}
