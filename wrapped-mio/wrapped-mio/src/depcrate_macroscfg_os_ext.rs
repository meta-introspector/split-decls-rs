// Generated macro for cfg_os_ext (macro)
macro_rules! Depcrate_macroscfg_os_ext {
() => {
// Module: crate::macros
// Provides: {"cfg_os_ext"}
// Dependencies: {}
# [doc = " The `os-ext` feature is enabled."] macro_rules ! cfg_os_ext { ($ ($ item : item) *) => { $ (# [cfg (feature = "os-ext")] # [cfg_attr (docsrs , doc (cfg (feature = "os-ext")))] $ item) * } }
};
}
