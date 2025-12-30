// Generated macro for impl_365 (impl)
macro_rules! Depcrate_ioimpl_365 {
() => {
// Module: crate::io
// Provides: {"impl_365"}
// Dependencies: {}
impl < T : AsyncRead + Unpin > AsyncRead for ReadHalf < T > { fn poll_read (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & mut [u8] ,) -> Poll < Result < usize > > { let mut inner = self . 0 . lock () . unwrap () ; Pin :: new (& mut * inner) . poll_read (cx , buf) } fn poll_read_vectored (self : Pin < & mut Self > , cx : & mut Context < '_ > , bufs : & mut [IoSliceMut < '_ >] ,) -> Poll < Result < usize > > { let mut inner = self . 0 . lock () . unwrap () ; Pin :: new (& mut * inner) . poll_read_vectored (cx , bufs) } }
};
}
