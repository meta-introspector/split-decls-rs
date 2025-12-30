// Generated macro for non_wsl_open (function)
macro_rules! Depcrate_linux_and_morenon_wsl_open {
() => {
// Module: crate::linux_and_more
// Provides: {"non_wsl_open"}
// Dependencies: {}
fn non_wsl_open (path : & OsStr) -> Result < () , OpenError > { if open_with_system_xdg_open (path) . is_err () { open_with_internal_xdg_open (path) ? ; } Ok (()) }
};
}
