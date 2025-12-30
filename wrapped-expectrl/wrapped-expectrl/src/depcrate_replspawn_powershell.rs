// Generated macro for spawn_powershell (function)
macro_rules! Depcrate_replspawn_powershell {
() => {
// Module: crate::repl
// Provides: {"spawn_powershell"}
// Dependencies: {}
# [doc = " Spawn a powershell session."] # [doc = ""] # [doc = " It uses a custom prompt to be able to controll the shell."] # [cfg (windows)] # [cfg (feature = "async")] pub async fn spawn_powershell () -> Result < ReplSession < OsSession > , Error > { const DEFAULT_PROMPT : & str = "EXPECTED_PROMPT>" ; let session = spawn ("pwsh -NoProfile -NonInteractive -NoLogo") ? ; let mut powershell = ReplSession :: new (session , DEFAULT_PROMPT) ; powershell . set_quit_command ("exit") ; powershell . set_echo (true) ; let _ = powershell . execute (format ! (r#"function prompt {{ "{}"; return " " }}"# , DEFAULT_PROMPT)) . await ? ; let _ = powershell . execute (r#"[System.Environment]::SetEnvironmentVariable("TERM", "dumb")"#) . await ? ; let _ = powershell . execute (r#"[System.Environment]::SetEnvironmentVariable("TERM", "NO_COLOR")"#) . await ? ; Ok (powershell) }
};
}
