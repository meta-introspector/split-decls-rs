// Generated macro for open (function)
macro_rules! Depcrate_linux_and_moreopen {
() => {
// Module: crate::linux_and_more
// Provides: {"open"}
// Dependencies: {}
pub (crate) fn open (path : & OsStr) -> Result < () , OpenError > { if crate :: is_wsl () { wsl_open (path) } else { non_wsl_open (path) } }
};
}
