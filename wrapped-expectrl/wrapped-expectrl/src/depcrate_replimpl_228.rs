// Generated macro for impl_228 (impl)
macro_rules! Depcrate_replimpl_228 {
() => {
// Module: crate::repl
// Provides: {"impl_228"}
// Dependencies: {}
# [cfg (not (feature = "async"))] impl < S > ReplSession < S > where S : Expect , { # [doc = " Send a command to a repl and verifies that it exited."] # [doc = " Returning it's output."] pub fn execute < C > (& mut self , cmd : C) -> Result < Vec < u8 > , Error > where C : AsRef < str > , { self . send_line (cmd) ? ; let found = self . _expect_prompt () ? ; let out = found . before () . to_vec () ; Ok (out) } # [doc = " Sends line to repl (and flush the output)."] # [doc = ""] # [doc = " If echo_on=true wait for the input to appear."] # [cfg (not (feature = "async"))] pub fn send_line < L > (& mut self , line : L) -> Result < () , Error > where L : AsRef < str > , { let text = line . as_ref () ; self . session . send_line (text) ? ; if self . is_echo_on { let _ = self . get_session_mut () . expect (line . as_ref ()) ? ; } Ok (()) } # [doc = " Send a quit command."] # [doc = ""] # [doc = " In async version we it won't be send on Drop so,"] # [doc = " If you wan't it to be send you must do it yourself."] pub fn exit (& mut self) -> Result < () , Error > { if let Some (quit_command) = & self . quit_command { self . session . send_line (quit_command) ? ; } Ok (()) } }
};
}
