// Generated macro for impl_498 (impl)
macro_rules! Depcrate_rt_tokioimpl_498 {
() => {
// Module: crate::rt::tokio
// Provides: {"impl_498"}
// Dependencies: {}
impl < T > hyper :: rt :: Write for TokioIo < T > where T : tokio :: io :: AsyncWrite , { fn poll_write (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & [u8] ,) -> Poll < Result < usize , std :: io :: Error > > { tokio :: io :: AsyncWrite :: poll_write (self . project () . inner , cx , buf) } fn poll_flush (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , std :: io :: Error > > { tokio :: io :: AsyncWrite :: poll_flush (self . project () . inner , cx) } fn poll_shutdown (self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> Poll < Result < () , std :: io :: Error > > { tokio :: io :: AsyncWrite :: poll_shutdown (self . project () . inner , cx) } fn is_write_vectored (& self) -> bool { tokio :: io :: AsyncWrite :: is_write_vectored (& self . inner) } fn poll_write_vectored (self : Pin < & mut Self > , cx : & mut Context < '_ > , bufs : & [std :: io :: IoSlice < '_ >] ,) -> Poll < Result < usize , std :: io :: Error > > { tokio :: io :: AsyncWrite :: poll_write_vectored (self . project () . inner , cx , bufs) } }
};
}
