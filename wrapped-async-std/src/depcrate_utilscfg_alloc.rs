// Generated macro for cfg_alloc (macro)
macro_rules! Depcrate_utilscfg_alloc {
() => {
// Module: crate::utils
// Provides: {"cfg_alloc"}
// Dependencies: {}
# [doc = " Declares no-std items."] # [allow (unused_macros)] # [doc (hidden)] macro_rules ! cfg_alloc { ($ ($ item : item) *) => { $ (# [cfg (feature = "alloc")] $ item) * } }
};
}
