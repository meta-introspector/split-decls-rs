// Generated macro for cfg_not_os_poll (macro)
macro_rules! Depcrate_macroscfg_not_os_poll {
() => {
// Module: crate::macros
// Provides: {"cfg_not_os_poll"}
// Dependencies: {}
# [doc = " The `os-poll` feature is disabled."] macro_rules ! cfg_not_os_poll { ($ ($ item : item) *) => { $ (# [cfg (not (feature = "os-poll"))] $ item) * } }
};
}
