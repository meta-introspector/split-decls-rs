// Generated macro for impl_181 (impl)
macro_rules! Depcrate_client_blocking_io_requestimpl_181 {
() => {
// Module: crate::client::blocking_io::request
// Provides: {"impl_181"}
// Dependencies: {}
impl io :: Write for RequestWriter < '_ > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { # [allow (unused_imports)] if self . trace { use bstr :: ByteSlice ; gix_features :: trace :: trace ! (">> {}" , buf . as_bstr ()) ; } self . writer . write (buf) } fn flush (& mut self) -> io :: Result < () > { self . writer . flush () } }
};
}
