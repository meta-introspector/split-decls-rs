// Generated macro for impl_276 (impl)
macro_rules! Depcrate_ioimpl_276 {
() => {
// Module: crate::io
// Provides: {"impl_276"}
// Dependencies: {}
impl AsyncWrite for Cursor < & mut [u8] > { fn poll_write (mut self : Pin < & mut Self > , _ : & mut Context < '_ > , buf : & [u8] ,) -> Poll < Result < usize > > { Poll :: Ready (std :: io :: Write :: write (& mut self . inner , buf)) } fn poll_write_vectored (mut self : Pin < & mut Self > , _ : & mut Context < '_ > , bufs : & [IoSlice < '_ >] ,) -> Poll < Result < usize > > { Poll :: Ready (std :: io :: Write :: write_vectored (& mut self . inner , bufs)) } fn poll_flush (mut self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < Result < () > > { Poll :: Ready (std :: io :: Write :: flush (& mut self . inner)) } fn poll_close (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () > > { self . poll_flush (cx) } }
};
}
