// Generated macro for CloseFuture (struct)
macro_rules! Depcrate_ioCloseFuture {
() => {
// Module: crate::io
// Provides: {"CloseFuture"}
// Dependencies: {}
# [doc = " Future for the [`AsyncWriteExt::close()`] method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct CloseFuture < 'a , W : Unpin + ? Sized > { writer : & 'a mut W , }
};
}
