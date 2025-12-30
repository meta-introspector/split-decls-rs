// Generated macro for impl_2404 (impl)
macro_rules! Depcrate_io_read_to_endimpl_2404 {
() => {
// Module: crate::io::read_to_end
// Provides: {"impl_2404"}
// Dependencies: {}
impl < A > Future for ReadToEnd < '_ , A > where A : AsyncRead + ? Sized + Unpin , { type Output = io :: Result < usize > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = & mut * self ; read_to_end_internal (Pin :: new (& mut this . reader) , cx , this . buf , this . start_len) } }
};
}
