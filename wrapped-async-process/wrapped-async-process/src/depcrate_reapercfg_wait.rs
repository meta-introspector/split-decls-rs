// Generated macro for cfg_wait (macro)
macro_rules! Depcrate_reapercfg_wait {
() => {
// Module: crate::reaper
// Provides: {"cfg_wait"}
// Dependencies: {}
# [doc = " Enable the waiting reaper."] # [cfg (not (any (windows , target_os = "linux")))] macro_rules ! cfg_wait { ($ ($ tt : tt) *) => { } ; }
};
}
