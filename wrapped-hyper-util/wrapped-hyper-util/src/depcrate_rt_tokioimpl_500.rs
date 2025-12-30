// Generated macro for impl_500 (impl)
macro_rules! Depcrate_rt_tokioimpl_500 {
() => {
// Module: crate::rt::tokio
// Provides: {"impl_500"}
// Dependencies: {}
impl < T > tokio :: io :: AsyncWrite for TokioIo < T > where T : hyper :: rt :: Write , { fn poll_write (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & [u8] ,) -> Poll < Result < usize , std :: io :: Error > > { hyper :: rt :: Write :: poll_write (self . project () . inner , cx , buf) } fn poll_flush (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , std :: io :: Error > > { hyper :: rt :: Write :: poll_flush (self . project () . inner , cx) } fn poll_shutdown (self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> Poll < Result < () , std :: io :: Error > > { hyper :: rt :: Write :: poll_shutdown (self . project () . inner , cx) } fn is_write_vectored (& self) -> bool { hyper :: rt :: Write :: is_write_vectored (& self . inner) } fn poll_write_vectored (self : Pin < & mut Self > , cx : & mut Context < '_ > , bufs : & [std :: io :: IoSlice < '_ >] ,) -> Poll < Result < usize , std :: io :: Error > > { hyper :: rt :: Write :: poll_write_vectored (self . project () . inner , cx , bufs) } }
};
}
