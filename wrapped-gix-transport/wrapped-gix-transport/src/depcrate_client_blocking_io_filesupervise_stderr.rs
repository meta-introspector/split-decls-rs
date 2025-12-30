// Generated macro for supervise_stderr (function)
macro_rules! Depcrate_client_blocking_io_filesupervise_stderr {
() => {
// Module: crate::client::blocking_io::file
// Provides: {"supervise_stderr"}
// Dependencies: {}
fn supervise_stderr (ssh_kind : ssh :: ProgramKind , stderr : std :: process :: ChildStderr , stdout : std :: process :: ChildStdout ,) -> ReadStdoutFailOnError { let (send , recv) = std :: sync :: mpsc :: sync_channel (1) ; std :: thread :: Builder :: new () . name ("supervise ssh stderr" . into ()) . stack_size (128 * 1024) . spawn (move | | -> std :: io :: Result < () > { let mut process_stderr = std :: io :: stderr () ; for line in std :: io :: BufReader :: new (stderr) . byte_lines () { let line = line ? ; match ssh_kind . line_to_err (line . into ()) { Ok (err) => { send . send (err) . ok () ; } Err (line) => { process_stderr . write_all (& line) . ok () ; writeln ! (& process_stderr) . ok () ; } } } Ok (()) }) . expect ("named threads with small stack work on all platforms") ; ReadStdoutFailOnError { read : stdout , recv } }
};
}
