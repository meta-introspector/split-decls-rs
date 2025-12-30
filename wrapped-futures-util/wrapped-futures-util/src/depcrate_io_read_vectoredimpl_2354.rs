// Generated macro for impl_2354 (impl)
macro_rules! Depcrate_io_read_vectoredimpl_2354 {
() => {
// Module: crate::io::read_vectored
// Provides: {"impl_2354"}
// Dependencies: {}
impl < 'a , 'b , R : AsyncRead + ? Sized + Unpin > ReadVectored < 'a , 'b , R > { pub (super) fn new (reader : & 'a mut R , bufs : & 'a mut [IoSliceMut < 'b >]) -> Self { Self { reader , bufs } } }
};
}
