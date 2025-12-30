// Generated macro for impl_327 (impl)
macro_rules! Depcrate_ioimpl_327 {
() => {
// Module: crate::io
// Provides: {"impl_327"}
// Dependencies: {}
impl < R : AsyncRead > AsyncRead for Take < R > { fn poll_read (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & mut [u8] ,) -> Poll < Result < usize > > { let this = self . project () ; take_read_internal (this . inner , cx , buf , this . limit) } }
};
}
