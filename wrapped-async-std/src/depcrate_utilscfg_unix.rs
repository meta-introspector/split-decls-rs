// Generated macro for cfg_unix (macro)
macro_rules! Depcrate_utilscfg_unix {
() => {
// Module: crate::utils
// Provides: {"cfg_unix"}
// Dependencies: {}
# [doc = " Declares Unix-specific items."] # [doc (hidden)] # [allow (unused_macros)] macro_rules ! cfg_unix { ($ ($ item : item) *) => { $ (# [cfg (any (unix , feature = "docs"))] $ item) * } }
};
}
