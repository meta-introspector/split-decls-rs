// Generated macro for impl_2400 (impl)
macro_rules! Depcrate_io_read_to_endimpl_2400 {
() => {
// Module: crate::io::read_to_end
// Provides: {"impl_2400"}
// Dependencies: {}
impl < 'a , R : AsyncRead + ? Sized + Unpin > ReadToEnd < 'a , R > { pub (super) fn new (reader : & 'a mut R , buf : & 'a mut Vec < u8 >) -> Self { let start_len = buf . len () ; Self { reader , buf , start_len } } }
};
}
