// Generated macro for WriterDrain (struct)
macro_rules! Depcrate_ioWriterDrain {
() => {
// Module: crate::io
// Provides: {"WriterDrain"}
// Dependencies: {}
struct WriterDrain < T , E > where T : AsyncWrite + Unpin , E : From < io :: Error > , { inner : UnsafeWriter < T , E > , }
};
}
