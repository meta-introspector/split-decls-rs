// Generated macro for impl_156 (impl)
macro_rules! Depcrate_process_uniximpl_156 {
() => {
// Module: crate::process::unix
// Provides: {"impl_156"}
// Dependencies: {}
impl Process for UnixProcess { type Command = Command ; type Stream = PtyStream ; fn spawn < S > (cmd : S) -> Result < Self > where S : AsRef < str > , { let args = tokenize_command (cmd . as_ref ()) ; if args . is_empty () { return Err (io_error ("failed to parse a command")) ; } let mut command = Command :: new (& args [0]) ; let _ = command . args (args . iter () . skip (1)) ; Self :: spawn_command (command) } fn spawn_command (command : Self :: Command) -> Result < Self > { let proc = PtyProcess :: spawn (command) . map_err (to_io_error ("Failed to spawn a command")) ? ; Ok (Self { proc }) } fn open_stream (& mut self) -> Result < Self :: Stream > { let stream = self . proc . get_pty_stream () . map_err (to_io_error ("Failed to create a stream")) ? ; let stream = PtyStream :: new (stream) ; Ok (stream) } }
};
}
