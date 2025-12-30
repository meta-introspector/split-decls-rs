// Generated macro for cfg_os_poll (macro)
macro_rules! Depcrate_macroscfg_os_poll {
() => {
// Module: crate::macros
// Provides: {"cfg_os_poll"}
// Dependencies: {}
# [doc = " The `os-poll` feature is enabled."] macro_rules ! cfg_os_poll { ($ ($ item : item) *) => { $ (# [cfg (feature = "os-poll")] # [cfg_attr (docsrs , doc (cfg (feature = "os-poll")))] $ item) * } }
};
}
