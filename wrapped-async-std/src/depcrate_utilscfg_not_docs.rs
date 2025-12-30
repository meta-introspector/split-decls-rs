// Generated macro for cfg_not_docs (macro)
macro_rules! Depcrate_utilscfg_not_docs {
() => {
// Module: crate::utils
// Provides: {"cfg_not_docs"}
// Dependencies: {}
# [doc = " Declares items when the \"docs\" feature is disabled."] # [doc (hidden)] # [allow (unused_macros)] macro_rules ! cfg_not_docs { ($ ($ item : item) *) => { $ (# [cfg (not (feature = "docs"))] $ item) * } }
};
}
