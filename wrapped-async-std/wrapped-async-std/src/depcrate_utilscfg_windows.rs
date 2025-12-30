// Generated macro for cfg_windows (macro)
macro_rules! Depcrate_utilscfg_windows {
() => {
// Module: crate::utils
// Provides: {"cfg_windows"}
// Dependencies: {}
# [doc = " Declares Windows-specific items."] # [doc (hidden)] # [allow (unused_macros)] macro_rules ! cfg_windows { ($ ($ item : item) *) => { $ (# [cfg (any (windows , feature = "docs"))] $ item) * } }
};
}
