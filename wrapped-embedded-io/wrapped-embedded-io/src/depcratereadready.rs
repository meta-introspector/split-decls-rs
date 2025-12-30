// Generated macro for ReadReady (trait)
macro_rules! DepcrateReadReady {
() => {
// Module: crate
// Provides: {"ReadReady"}
// Dependencies: {}
# [doc = " Get whether a reader is ready."] # [doc = ""] # [doc = " This allows using a [`Read`] or [`BufRead`] in a nonblocking fashion, i.e. trying to read"] # [doc = " only when it is ready."] pub trait ReadReady : ErrorType { # [doc = " Get whether the reader is ready for immediately reading."] # [doc = ""] # [doc = " This usually means that there is either some bytes have been received and are buffered and ready to be read,"] # [doc = " or that the reader is at EOF."] # [doc = ""] # [doc = " If this returns `true`, it's guaranteed that the next call to [`Read::read`] or [`BufRead::fill_buf`] will not block."] fn read_ready (& mut self) -> Result < bool , Self :: Error > ; }
};
}
