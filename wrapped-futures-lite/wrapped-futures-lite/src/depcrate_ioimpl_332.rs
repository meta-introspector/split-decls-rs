// Generated macro for impl_332 (impl)
macro_rules! Depcrate_ioimpl_332 {
() => {
// Module: crate::io
// Provides: {"impl_332"}
// Dependencies: {}
impl < R : AsyncRead > AsyncRead for Bytes < R > { fn poll_read (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & mut [u8] ,) -> Poll < Result < usize > > { self . project () . inner . poll_read (cx , buf) } fn poll_read_vectored (self : Pin < & mut Self > , cx : & mut Context < '_ > , bufs : & mut [IoSliceMut < '_ >] ,) -> Poll < Result < usize > > { self . project () . inner . poll_read_vectored (cx , bufs) } }
};
}
