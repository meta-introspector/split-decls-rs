// Generated macro for impl_2292 (impl)
macro_rules! Depcrate_io_fill_bufimpl_2292 {
() => {
// Module: crate::io::fill_buf
// Provides: {"impl_2292"}
// Dependencies: {}
impl < 'a , R : AsyncBufRead + ? Sized + Unpin > FillBuf < 'a , R > { pub (super) fn new (reader : & 'a mut R) -> Self { Self { reader : Some (reader) } } }
};
}
