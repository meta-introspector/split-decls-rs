// Generated macro for cfg_net (macro)
macro_rules! Depcrate_macroscfg_net {
() => {
// Module: crate::macros
// Provides: {"cfg_net"}
// Dependencies: {}
# [doc = " The `net` feature is enabled."] macro_rules ! cfg_net { ($ ($ item : item) *) => { $ (# [cfg (feature = "net")] # [cfg_attr (docsrs , doc (cfg (feature = "net")))] $ item) * } }
};
}
