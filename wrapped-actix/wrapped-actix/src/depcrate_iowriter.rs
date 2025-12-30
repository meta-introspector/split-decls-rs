// Generated macro for Writer (struct)
macro_rules! Depcrate_ioWriter {
() => {
// Module: crate::io
// Provides: {"Writer"}
// Dependencies: {}
# [doc = " A wrapper for `AsyncWrite` types."] pub struct Writer < T : AsyncWrite , E : From < io :: Error > > { inner : UnsafeWriter < T , E > , }
};
}
