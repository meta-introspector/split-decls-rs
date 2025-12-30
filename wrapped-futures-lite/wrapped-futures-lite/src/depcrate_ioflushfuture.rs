// Generated macro for FlushFuture (struct)
macro_rules! Depcrate_ioFlushFuture {
() => {
// Module: crate::io
// Provides: {"FlushFuture"}
// Dependencies: {}
# [doc = " Future for the [`AsyncWriteExt::flush()`] method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct FlushFuture < 'a , W : Unpin + ? Sized > { writer : & 'a mut W , }
};
}
