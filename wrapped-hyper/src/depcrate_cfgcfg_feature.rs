// Generated macro for cfg_feature (macro)
macro_rules! Depcrate_cfgcfg_feature {
() => {
// Module: crate::cfg
// Provides: {"cfg_feature"}
// Dependencies: {}
macro_rules ! cfg_feature { (#! [$ meta : meta] $ ($ item : item) *) => { $ (# [cfg ($ meta)] # [cfg_attr (docsrs , doc (cfg ($ meta)))] $ item) * } }
};
}
