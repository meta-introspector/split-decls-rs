// Generated macro for cfg_unstable (macro)
macro_rules! Depcrate_utilscfg_unstable {
() => {
// Module: crate::utils
// Provides: {"cfg_unstable"}
// Dependencies: {}
# [doc = " Declares unstable items."] # [doc (hidden)] macro_rules ! cfg_unstable { ($ ($ item : item) *) => { $ (# [cfg (feature = "unstable")] # [cfg_attr (feature = "docs" , doc (cfg (unstable)))] $ item) * } }
};
}
