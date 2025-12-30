// Generated macro for impl_2384 (impl)
macro_rules! Depcrate_io_read_lineimpl_2384 {
() => {
// Module: crate::io::read_line
// Provides: {"impl_2384"}
// Dependencies: {}
impl < 'a , R : AsyncBufRead + ? Sized + Unpin > ReadLine < 'a , R > { pub (super) fn new (reader : & 'a mut R , buf : & 'a mut String) -> Self { Self { reader , bytes : mem :: take (buf) . into_bytes () , buf , read : 0 , finished : false } } }
};
}
