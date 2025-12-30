// Generated macro for cfg_proto (macro)
macro_rules! Depcrate_cfgcfg_proto {
() => {
// Module: crate::cfg
// Provides: {"cfg_proto"}
// Dependencies: {}
macro_rules ! cfg_proto { ($ ($ item : item) *) => { cfg_feature ! { #! [all (any (feature = "http1" , feature = "http2") , any (feature = "client" , feature = "server") ,)] $ ($ item) * } } }
};
}
