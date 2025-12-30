// Generated macro for impl_2557 (impl)
macro_rules! Depcrate_io_write_all_vectoredimpl_2557 {
() => {
// Module: crate::io::write_all_vectored
// Provides: {"impl_2557"}
// Dependencies: {}
impl < 'a , 'b , W : AsyncWrite + ? Sized + Unpin > WriteAllVectored < 'a , 'b , W > { pub (super) fn new (writer : & 'a mut W , mut bufs : & 'a mut [IoSlice < 'b >]) -> Self { IoSlice :: advance_slices (& mut bufs , 0) ; Self { writer , bufs } } }
};
}
