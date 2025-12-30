// Generated macro for impl_2520 (impl)
macro_rules! Depcrate_io_writeimpl_2520 {
() => {
// Module: crate::io::write
// Provides: {"impl_2520"}
// Dependencies: {}
impl < 'a , W : AsyncWrite + ? Sized + Unpin > Write < 'a , W > { pub (super) fn new (writer : & 'a mut W , buf : & 'a [u8]) -> Self { Self { writer , buf } } }
};
}
