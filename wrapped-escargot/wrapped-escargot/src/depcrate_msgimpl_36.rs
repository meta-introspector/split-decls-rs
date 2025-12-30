// Generated macro for impl_36 (impl)
macro_rules! Depcrate_msgimpl_36 {
() => {
// Module: crate::msg
// Provides: {"impl_36"}
// Dependencies: {}
impl CommandMessages { # [doc = " Run the command, allowing iteration over ndjson messages."] pub fn with_command (mut cmd : process :: Command) -> CargoResult < Self > { let mut child = cmd . stdout (process :: Stdio :: piped ()) . stderr (process :: Stdio :: piped ()) . spawn () . map_err (| e | CargoError :: new (ErrorKind :: InvalidCommand) . set_cause (e)) ? ; let stdout = child . stdout . take () . expect ("piped above") ; let stdout = io :: BufReader :: new (stdout) ; let stderr = child . stderr . take () . expect ("piped above") ; let stderr = io :: BufReader :: new (stderr) ; let msgs = InnerCommandMessages { done : false , child , stdout , stderr , } ; Ok (CommandMessages (msgs)) } # [inline] fn next_msg (& mut self) -> CargoResult < Option < Message > > { # ! [allow (clippy :: branches_sharing_code)] let mut content = String :: new () ; let len = self . 0 . stdout . read_line (& mut content) . map_err (| e | CargoError :: new (ErrorKind :: InvalidOutput) . set_cause (e)) ? ; if 0 < len { Ok (Some (Message (content))) } else { let status = self . 0 . child . wait () . map_err (| e | CargoError :: new (ErrorKind :: InvalidOutput) . set_cause (e)) ? ; if ! status . success () && ! self . 0 . done { self . 0 . done = true ; let mut data = vec ! [] ; self . 0 . stderr . read_to_end (& mut data) . map_err (| e | CargoError :: new (ErrorKind :: InvalidOutput) . set_cause (e)) ? ; let err = CargoError :: new (ErrorKind :: CommandFailed) . set_context (String :: from_utf8_lossy (& data)) ; Err (err) } else { self . 0 . done = true ; Ok (None) } } } }
};
}
