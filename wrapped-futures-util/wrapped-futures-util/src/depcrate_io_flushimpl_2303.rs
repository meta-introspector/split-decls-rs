// Generated macro for impl_2303 (impl)
macro_rules! Depcrate_io_flushimpl_2303 {
() => {
// Module: crate::io::flush
// Provides: {"impl_2303"}
// Dependencies: {}
impl < 'a , W : AsyncWrite + ? Sized + Unpin > Flush < 'a , W > { pub (super) fn new (writer : & 'a mut W) -> Self { Self { writer } } }
};
}
