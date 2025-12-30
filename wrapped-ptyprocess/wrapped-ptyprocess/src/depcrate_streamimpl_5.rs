// Generated macro for impl_5 (impl)
macro_rules! Depcrate_streamimpl_5 {
() => {
// Module: crate::stream
// Provides: {"impl_5"}
// Dependencies: {}
impl Write for Stream { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . inner . write (buf) } fn flush (& mut self) -> io :: Result < () > { self . inner . flush () } fn write_vectored (& mut self , bufs : & [io :: IoSlice < '_ >]) -> io :: Result < usize > { self . inner . write_vectored (bufs) } }
};
}
