// Generated macro for hermit (module)
macro_rules! Depcratehermit {
() => {
// Module: crate
// Provides: {"hermit"}
// Dependencies: {}
# [cfg (all (target_os = "hermit" , feature = "os-ext"))] # [cfg_attr (docsrs , doc (cfg (all (target_os = "hermit" , feature = "os-ext"))))] pub mod hermit { # ! [doc = " Hermit only extensions."] pub use crate :: sys :: SourceFd ; }
};
}
