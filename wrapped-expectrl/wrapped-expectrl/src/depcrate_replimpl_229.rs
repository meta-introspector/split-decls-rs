// Generated macro for impl_229 (impl)
macro_rules! Depcrate_replimpl_229 {
() => {
// Module: crate::repl
// Provides: {"impl_229"}
// Dependencies: {}
# [cfg (feature = "async")] impl < S > ReplSession < S > where S : AsyncExpect + Unpin , { # [doc = " Send a command to a repl and verifies that it exited."] pub async fn execute (& mut self , cmd : impl AsRef < str >) -> Result < Vec < u8 > , Error > { self . send_line (cmd) . await ? ; let found = self . _expect_prompt () . await ? ; Ok (found . before () . to_vec ()) } # [doc = " Sends line to repl (and flush the output)."] # [doc = ""] # [doc = " If echo_on=true wait for the input to appear."] pub async fn send_line (& mut self , line : impl AsRef < str >) -> Result < () , Error > { self . session . send_line (line . as_ref ()) . await ? ; if self . is_echo_on { let _ = self . expect (line . as_ref ()) . await ? ; } Ok (()) } # [doc = " Send a quit command."] # [doc = ""] # [doc = " In async version we it won't be send on Drop so,"] # [doc = " If you wan't it to be send you must do it yourself."] pub async fn exit (& mut self) -> Result < () , Error > { if let Some (quit_command) = & self . quit_command { self . session . send_line (quit_command) . await ? ; } Ok (()) } }
};
}
