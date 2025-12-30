// Generated macro for WriteReady (trait)
macro_rules! DepcrateWriteReady {
() => {
// Module: crate
// Provides: {"WriteReady"}
// Dependencies: {}
# [doc = " Get whether a writer is ready."] # [doc = ""] # [doc = " This allows using a [`Write`] in a nonblocking fashion, i.e. trying to write"] # [doc = " only when it is ready."] pub trait WriteReady : ErrorType { # [doc = " Get whether the writer is ready for immediately writing."] # [doc = ""] # [doc = " This usually means that there is free space in the internal transmit buffer."] # [doc = ""] # [doc = " If this returns `true`, it's guaranteed that the next call to [`Write::write`] will not block."] fn write_ready (& mut self) -> Result < bool , Self :: Error > ; }
};
}
