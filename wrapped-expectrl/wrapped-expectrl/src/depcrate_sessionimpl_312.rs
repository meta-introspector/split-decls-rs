// Generated macro for impl_312 (impl)
macro_rules! Depcrate_sessionimpl_312 {
() => {
// Module: crate::session
// Provides: {"impl_312"}
// Dependencies: {}
impl Session < OsProc , OsProcStream > { # [doc = " Spawns a session on a platform process."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::process::Command;"] # [doc = " use expectrl::Session;"] # [doc = ""] # [doc = " let p = Session::spawn(Command::new(\"cat\"));"] # [doc = " ```"] pub fn spawn (command : Command) -> Result < Self , Error > { let mut process = OsProcess :: spawn_command (command) ? ; let stream = process . open_stream () ? ; # [cfg (feature = "async")] let stream = stream . into_async_stream () ? ; let session = Self :: new (process , stream) ? ; Ok (session) } # [doc = " Spawns a session on a platform process."] # [doc = " Using a string commandline."] pub (crate) fn spawn_cmd (cmd : & str) -> Result < Self , Error > { let mut process = OsProcess :: spawn (cmd) ? ; let stream = process . open_stream () ? ; # [cfg (feature = "async")] let stream = stream . into_async_stream () ? ; let session = Self :: new (process , stream) ? ; Ok (session) } }
};
}
