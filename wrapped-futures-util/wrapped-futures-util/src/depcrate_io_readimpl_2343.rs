// Generated macro for impl_2343 (impl)
macro_rules! Depcrate_io_readimpl_2343 {
() => {
// Module: crate::io::read
// Provides: {"impl_2343"}
// Dependencies: {}
impl < 'a , R : AsyncRead + ? Sized + Unpin > Read < 'a , R > { pub (super) fn new (reader : & 'a mut R , buf : & 'a mut [u8]) -> Self { Self { reader , buf } } }
};
}
