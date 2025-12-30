// Generated macro for spawn (function)
macro_rules! Depcratespawn {
() => {
// Module: crate
// Provides: {"spawn"}
// Dependencies: {}
# [doc = " Spawn spawnes a new session."] # [doc = ""] # [doc = " It accepts a command and possibly arguments just as string."] # [doc = " It doesn't parses ENV variables. For complex constrictions use [`Session::spawn`]."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```no_run,ignore"] # [doc = " use std::{thread, time::Duration, io::{Read, Write}};"] # [doc = " use expectrl::{spawn, ControlCode};"] # [doc = ""] # [doc = " let mut p = spawn(\"cat\").unwrap();"] # [doc = " p.send_line(\"Hello World\").unwrap();"] # [doc = ""] # [doc = " thread::sleep(Duration::from_millis(300)); // give 'cat' some time to set up"] # [doc = " p.send(ControlCode::EndOfText).unwrap(); // abort: SIGINT"] # [doc = ""] # [doc = " let mut buf = String::new();"] # [doc = " p.read_to_string(&mut buf).unwrap();"] # [doc = ""] # [doc = " assert_eq!(buf, \"Hello World\\r\\n\");"] # [doc = " ```"] # [doc = ""] # [doc = " [`Session::spawn`]: ./struct.Session.html?#spawn"] pub fn spawn < S > (cmd : S) -> Result < OsSession , Error > where S : AsRef < str > , { Session :: spawn_cmd (cmd . as_ref ()) }
};
}
