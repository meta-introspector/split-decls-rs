// Generated macro for WriterFut (struct)
macro_rules! Depcrate_ioWriterFut {
() => {
// Module: crate::io
// Provides: {"WriterFut"}
// Dependencies: {}
struct WriterFut < T , E > where T : AsyncWrite + Unpin , E : From < io :: Error > , { inner : UnsafeWriter < T , E > , }
};
}
