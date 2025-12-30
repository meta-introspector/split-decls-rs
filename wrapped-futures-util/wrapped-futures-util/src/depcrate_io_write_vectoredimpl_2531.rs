// Generated macro for impl_2531 (impl)
macro_rules! Depcrate_io_write_vectoredimpl_2531 {
() => {
// Module: crate::io::write_vectored
// Provides: {"impl_2531"}
// Dependencies: {}
impl < 'a , 'b , W : AsyncWrite + ? Sized + Unpin > WriteVectored < 'a , 'b , W > { pub (super) fn new (writer : & 'a mut W , bufs : & 'a [IoSlice < 'b >]) -> Self { Self { writer , bufs } } }
};
}
