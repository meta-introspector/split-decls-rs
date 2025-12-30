// Generated macro for WritableTarget (enum)
macro_rules! Depcrate_writer_bufferWritableTarget {
() => {
// Module: crate::writer::buffer
// Provides: {"WritableTarget"}
// Dependencies: {}
# [doc = " Log target, either `stdout`, `stderr` or a custom pipe."] # [doc = ""] # [doc = " Same as `Target`, except the pipe is wrapped in a mutex for interior mutability."] pub (crate) enum WritableTarget { # [doc = " Logs will be written to standard output."] WriteStdout , # [doc = " Logs will be printed to standard output."] PrintStdout , # [doc = " Logs will be written to standard error."] WriteStderr , # [doc = " Logs will be printed to standard error."] PrintStderr , # [doc = " Logs will be sent to a custom pipe."] Pipe (Box < Mutex < dyn io :: Write + Send + 'static > >) , }
};
}
