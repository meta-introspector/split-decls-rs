// Generated macro for impl_188 (impl)
macro_rules! Depcrate_process_windowsimpl_188 {
() => {
// Module: crate::process::windows
// Provides: {"impl_188"}
// Dependencies: {}
impl ProcessTrait for WinProcess { type Command = Command ; type Stream = ProcessStream ; fn spawn < S : AsRef < str > > (cmd : S) -> Result < Self > { spawn (cmd . as_ref ()) . map_err (to_io_error ("")) . map (| proc | WinProcess { proc }) } fn spawn_command (command : Self :: Command) -> Result < Self > { conpty :: Process :: spawn (command) . map_err (to_io_error ("")) . map (| proc | WinProcess { proc }) } fn open_stream (& mut self) -> Result < Self :: Stream > { let input = self . proc . input () . map_err (to_io_error ("")) ? ; let output = self . proc . output () . map_err (to_io_error ("")) ? ; Ok (Self :: Stream :: new (output , input)) } }
};
}
