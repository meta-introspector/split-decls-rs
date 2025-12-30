// Generated macro for impl_2211 (impl)
macro_rules! Depcrate_io_closeimpl_2211 {
() => {
// Module: crate::io::close
// Provides: {"impl_2211"}
// Dependencies: {}
impl < 'a , W : AsyncWrite + ? Sized + Unpin > Close < 'a , W > { pub (super) fn new (writer : & 'a mut W) -> Self { Self { writer } } }
};
}
