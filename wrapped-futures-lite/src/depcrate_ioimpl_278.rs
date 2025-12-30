// Generated macro for impl_278 (impl)
macro_rules! Depcrate_ioimpl_278 {
() => {
// Module: crate::io
// Provides: {"impl_278"}
// Dependencies: {}
impl AsyncWrite for Cursor < Vec < u8 > > { fn poll_write (mut self : Pin < & mut Self > , _ : & mut Context < '_ > , buf : & [u8] ,) -> Poll < Result < usize > > { Poll :: Ready (std :: io :: Write :: write (& mut self . inner , buf)) } fn poll_close (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () > > { self . poll_flush (cx) } fn poll_flush (mut self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < Result < () > > { Poll :: Ready (std :: io :: Write :: flush (& mut self . inner)) } }
};
}
