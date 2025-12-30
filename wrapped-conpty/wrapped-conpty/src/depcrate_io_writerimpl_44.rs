// Generated macro for impl_44 (impl)
macro_rules! Depcrate_io_writerimpl_44 {
() => {
// Module: crate::io::writer
// Provides: {"impl_44"}
// Dependencies: {}
impl From < PipeWriter > for std :: fs :: File { fn from (pipe : PipeWriter) -> Self { use std :: os :: windows :: io :: FromRawHandle ; let pipe = std :: mem :: ManuallyDrop :: new (pipe) ; unsafe { std :: fs :: File :: from_raw_handle (pipe . handle . 0 as _) } } }
};
}
