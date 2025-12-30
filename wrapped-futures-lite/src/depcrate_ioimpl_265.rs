// Generated macro for impl_265 (impl)
macro_rules! Depcrate_ioimpl_265 {
() => {
// Module: crate::io
// Provides: {"impl_265"}
// Dependencies: {}
impl < R : AsyncWrite > AsyncWrite for BufReader < R > { fn poll_write (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & [u8] ,) -> Poll < Result < usize > > { self . as_mut () . get_pin_mut () . poll_write (cx , buf) } fn poll_flush (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () > > { self . as_mut () . get_pin_mut () . poll_flush (cx) } fn poll_close (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () > > { self . as_mut () . get_pin_mut () . poll_close (cx) } }
};
}
