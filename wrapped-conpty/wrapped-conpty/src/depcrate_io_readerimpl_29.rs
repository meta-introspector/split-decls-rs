// Generated macro for impl_29 (impl)
macro_rules! Depcrate_io_readerimpl_29 {
() => {
// Module: crate::io::reader
// Provides: {"impl_29"}
// Dependencies: {}
impl From < PipeReader > for std :: fs :: File { fn from (pipe : PipeReader) -> Self { use std :: os :: windows :: io :: FromRawHandle ; let pipe = std :: mem :: ManuallyDrop :: new (pipe) ; unsafe { std :: fs :: File :: from_raw_handle (pipe . handle . 0 as _) } } }
};
}
