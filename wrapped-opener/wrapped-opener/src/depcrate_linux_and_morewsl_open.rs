// Generated macro for wsl_open (function)
macro_rules! Depcrate_linux_and_morewsl_open {
() => {
// Module: crate::linux_and_more
// Provides: {"wsl_open"}
// Dependencies: {}
fn wsl_open (path : & OsStr) -> Result < () , OpenError > { let result = open_with_wslview (path) ; if let Ok (mut child) = result { return crate :: wait_child (& mut child , "wslview") ; } open_with_system_xdg_open (path) . map_err (| err | OpenError :: Spawn { cmds : "wslview, xdg-open" . into () , source : err , }) ? ; Ok (()) }
};
}
