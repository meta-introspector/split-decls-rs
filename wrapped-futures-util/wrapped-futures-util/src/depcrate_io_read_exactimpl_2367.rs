// Generated macro for impl_2367 (impl)
macro_rules! Depcrate_io_read_exactimpl_2367 {
() => {
// Module: crate::io::read_exact
// Provides: {"impl_2367"}
// Dependencies: {}
impl < 'a , R : AsyncRead + ? Sized + Unpin > ReadExact < 'a , R > { pub (super) fn new (reader : & 'a mut R , buf : & 'a mut [u8]) -> Self { Self { reader , buf } } }
};
}
