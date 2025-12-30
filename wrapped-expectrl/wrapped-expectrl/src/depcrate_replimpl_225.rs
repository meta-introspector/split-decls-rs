// Generated macro for impl_225 (impl)
macro_rules! Depcrate_replimpl_225 {
() => {
// Module: crate::repl
// Provides: {"impl_225"}
// Dependencies: {}
impl < S > ReplSession < S > { # [doc = " Spawn function creates a repl session."] # [doc = ""] # [doc = " The argument list is:"] # [doc = "     - session; a spawned session which repl will wrap."] # [doc = "     - prompt; a string which will identify that the command was run."] # [doc = "     - quit_command; a command which will be called when [ReplSession] instance is dropped."] # [doc = "     - is_echo_on; determines whether the prompt check will be done twice."] pub fn new (session : S , prompt : impl Into < String >) -> Self { Self { session , prompt : prompt . into () , quit_command : None , is_echo_on : false , } } # [doc = " Set echo settings to be expected."] pub fn set_echo (& mut self , on : bool) { self . is_echo_on = on ; } # [doc = " Set quit command which will be called on `exit`."] pub fn set_quit_command (& mut self , cmd : impl Into < String >) { self . quit_command = Some (cmd . into ()) ; } # [doc = " Get a used prompt."] pub fn get_prompt (& self) -> & str { & self . prompt } # [doc = " Get a used quit command."] pub fn get_quit_command (& self) -> Option < & str > { self . quit_command . as_deref () } # [doc = " Get a echo settings."] pub fn is_echo (& self) -> bool { self . is_echo_on } # [doc = " Get an inner session."] pub fn into_session (self) -> S { self . session } # [doc = " Get an inner session."] pub fn get_session (& self) -> & S { & self . session } # [doc = " Get an inner session."] pub fn get_session_mut (& mut self) -> & mut S { & mut self . session } }
};
}
