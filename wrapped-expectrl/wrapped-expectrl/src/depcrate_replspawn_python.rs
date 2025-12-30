// Generated macro for spawn_python (function)
macro_rules! Depcrate_replspawn_python {
() => {
// Module: crate::repl
// Provides: {"spawn_python"}
// Dependencies: {}
# [doc = " Spawn default python's IDLE."] # [cfg (feature = "async")] pub async fn spawn_python () -> Result < ReplSession < OsSession > , Error > { let session = spawn ("python") ? ; let mut idle = ReplSession :: new (session , ">>> ") ; idle . set_quit_command ("quit()") ; idle . set_echo (false) ; idle . expect_prompt () . await ? ; Ok (idle) }
};
}
