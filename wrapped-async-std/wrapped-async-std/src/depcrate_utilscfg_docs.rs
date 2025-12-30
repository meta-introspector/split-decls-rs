// Generated macro for cfg_docs (macro)
macro_rules! Depcrate_utilscfg_docs {
() => {
// Module: crate::utils
// Provides: {"cfg_docs"}
// Dependencies: {}
# [doc = " Declares items when the \"docs\" feature is enabled."] # [doc (hidden)] # [allow (unused_macros)] macro_rules ! cfg_docs { ($ ($ item : item) *) => { $ (# [cfg (feature = "docs")] $ item) * } }
};
}
