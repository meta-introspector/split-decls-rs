// Generated macro for cfg_any_os_ext (macro)
macro_rules! Depcrate_macroscfg_any_os_ext {
() => {
// Module: crate::macros
// Provides: {"cfg_any_os_ext"}
// Dependencies: {}
# [doc = " The `os-ext` feature is enabled, or one of the features that need `os-ext`."] macro_rules ! cfg_any_os_ext { ($ ($ item : item) *) => { $ (# [cfg (any (feature = "os-ext" , feature = "net"))] # [cfg_attr (docsrs , doc (cfg (any (feature = "os-ext" , feature = "net"))))] $ item) * } }
};
}
