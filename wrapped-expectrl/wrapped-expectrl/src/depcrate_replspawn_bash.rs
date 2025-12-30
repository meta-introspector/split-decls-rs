// Generated macro for spawn_bash (function)
macro_rules! Depcrate_replspawn_bash {
() => {
// Module: crate::repl
// Provides: {"spawn_bash"}
// Dependencies: {}
# [doc = " Spawn a bash session."] # [doc = ""] # [doc = " It uses a custom prompt to be able to controll shell better."] # [cfg (unix)] # [cfg (feature = "async")] pub async fn spawn_bash () -> Result < ReplSession < OsSession > , Error > { const DEFAULT_PROMPT : & str = "EXPECT_PROMPT" ; let mut cmd = Command :: new ("bash") ; let _ = cmd . env ("PS1" , DEFAULT_PROMPT) ; let _ = cmd . env ("PROMPT_COMMAND" , "PS1=EXPECT_PROMPT; unset PROMPT_COMMAND; bind 'set enable-bracketed-paste off'" ,) ; let session = crate :: session :: Session :: spawn (cmd) ? ; let mut bash = ReplSession :: new (session , DEFAULT_PROMPT) ; bash . set_quit_command ("quit") ; bash . set_echo (false) ; bash . expect_prompt () . await ? ; Ok (bash) }
};
}
