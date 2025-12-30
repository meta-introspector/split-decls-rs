// Generated macro for impl_2486 (impl)
macro_rules! Depcrate_io_splitimpl_2486 {
() => {
// Module: crate::io::split
// Provides: {"impl_2486"}
// Dependencies: {}
impl < R : AsyncRead > AsyncRead for ReadHalf < R > { fn poll_read (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & mut [u8] ,) -> Poll < io :: Result < usize > > { lock_and_then (& self . handle , cx , | l , cx | l . poll_read (cx , buf)) } fn poll_read_vectored (self : Pin < & mut Self > , cx : & mut Context < '_ > , bufs : & mut [IoSliceMut < '_ >] ,) -> Poll < io :: Result < usize > > { lock_and_then (& self . handle , cx , | l , cx | l . poll_read_vectored (cx , bufs)) } }
};
}
