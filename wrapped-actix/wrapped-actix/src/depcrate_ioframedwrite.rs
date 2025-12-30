// Generated macro for FramedWrite (struct)
macro_rules! Depcrate_ioFramedWrite {
() => {
// Module: crate::io
// Provides: {"FramedWrite"}
// Dependencies: {}
# [doc = " A wrapper for the `AsyncWrite` and `Encoder` types. The [`AsyncWrite`] will be flushed when this"] # [doc = " struct is dropped."] pub struct FramedWrite < I , T : AsyncWrite + Unpin , U : Encoder < I > > { enc : U , inner : UnsafeWriter < T , U :: Error > , }
};
}
