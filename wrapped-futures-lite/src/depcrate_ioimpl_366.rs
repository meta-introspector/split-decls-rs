// Generated macro for impl_366 (impl)
macro_rules! Depcrate_ioimpl_366 {
() => {
// Module: crate::io
// Provides: {"impl_366"}
// Dependencies: {}
impl < T : AsyncWrite + Unpin > AsyncWrite for WriteHalf < T > { fn poll_write (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & [u8]) -> Poll < Result < usize > > { let mut inner = self . 0 . lock () . unwrap () ; Pin :: new (& mut * inner) . poll_write (cx , buf) } fn poll_flush (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () > > { let mut inner = self . 0 . lock () . unwrap () ; Pin :: new (& mut * inner) . poll_flush (cx) } fn poll_close (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () > > { let mut inner = self . 0 . lock () . unwrap () ; Pin :: new (& mut * inner) . poll_close (cx) } }
};
}
