// Generated macro for impl_2544 (impl)
macro_rules! Depcrate_io_write_allimpl_2544 {
() => {
// Module: crate::io::write_all
// Provides: {"impl_2544"}
// Dependencies: {}
impl < 'a , W : AsyncWrite + ? Sized + Unpin > WriteAll < 'a , W > { pub (super) fn new (writer : & 'a mut W , buf : & 'a [u8]) -> Self { Self { writer , buf } } }
};
}
