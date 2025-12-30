// Generated macro for impl_269 (impl)
macro_rules! Depcrate_ioimpl_269 {
() => {
// Module: crate::io
// Provides: {"impl_269"}
// Dependencies: {}
impl < W : AsyncWrite > AsyncWrite for BufWriter < W > { fn poll_write (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & [u8] ,) -> Poll < Result < usize > > { if self . buf . len () + buf . len () > self . buf . capacity () { ready ! (self . as_mut () . poll_flush_buf (cx)) ? ; } if buf . len () >= self . buf . capacity () { self . get_pin_mut () . poll_write (cx , buf) } else { Pin :: new (& mut * self . project () . buf) . poll_write (cx , buf) } } fn poll_flush (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () > > { ready ! (self . as_mut () . poll_flush_buf (cx)) ? ; self . get_pin_mut () . poll_flush (cx) } fn poll_close (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () > > { ready ! (self . as_mut () . poll_flush_buf (cx)) ? ; self . get_pin_mut () . poll_close (cx) } }
};
}
