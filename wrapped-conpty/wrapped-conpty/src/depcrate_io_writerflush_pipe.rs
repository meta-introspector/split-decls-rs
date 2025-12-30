// Generated macro for flush_pipe (function)
macro_rules! Depcrate_io_writerflush_pipe {
() => {
// Module: crate::io::writer
// Provides: {"flush_pipe"}
// Dependencies: {}
fn flush_pipe (h : HANDLE) -> Result < () , io :: Error > { unsafe { FlushFileBuffers (h) ? ; } Ok (()) }
};
}
