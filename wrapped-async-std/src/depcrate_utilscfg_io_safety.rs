// Generated macro for cfg_io_safety (macro)
macro_rules! Depcrate_utilscfg_io_safety {
() => {
// Module: crate::utils
// Provides: {"cfg_io_safety"}
// Dependencies: {}
# [doc = " Declares items that use I/O safety."] # [allow (unused_macros)] # [doc (hidden)] macro_rules ! cfg_io_safety { ($ ($ item : item) *) => { $ (# [cfg (feature = "io_safety")] $ item) * } }
};
}
