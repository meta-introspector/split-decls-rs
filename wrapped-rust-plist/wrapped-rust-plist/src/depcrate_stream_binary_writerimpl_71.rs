// Generated macro for impl_71 (impl)
macro_rules! Depcrate_stream_binary_writerimpl_71 {
() => {
// Module: crate::stream::binary_writer
// Provides: {"impl_71"}
// Dependencies: {}
impl < W : Write > Write for PosWriter < W > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { let count = self . writer . write (buf) ? ; self . pos = self . pos . checked_add (count) . expect ("binary plist cannot be larger than `usize::MAX` bytes") ; Ok (count) } fn flush (& mut self) -> io :: Result < () > { self . writer . flush () } }
};
}
