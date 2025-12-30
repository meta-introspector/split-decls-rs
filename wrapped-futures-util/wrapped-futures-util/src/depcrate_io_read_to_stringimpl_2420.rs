// Generated macro for impl_2420 (impl)
macro_rules! Depcrate_io_read_to_stringimpl_2420 {
() => {
// Module: crate::io::read_to_string
// Provides: {"impl_2420"}
// Dependencies: {}
impl < A > Future for ReadToString < '_ , A > where A : AsyncRead + ? Sized + Unpin , { type Output = io :: Result < usize > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let Self { reader , buf , bytes , start_len } = & mut * self ; read_to_string_internal (Pin :: new (reader) , cx , buf , bytes , * start_len) } }
};
}
