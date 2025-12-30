// Generated macro for PipeReader (struct)
macro_rules! Depcrate_io_readerPipeReader {
() => {
// Module: crate::io::reader
// Provides: {"PipeReader"}
// Dependencies: {}
# [doc = " PipeReader wraps a win32 pipe to provide a [std::io::Read] interface."] # [doc = " It also provides a non_blocking mode settings."] pub struct PipeReader { handle : HANDLE , blocking : bool , }
};
}
