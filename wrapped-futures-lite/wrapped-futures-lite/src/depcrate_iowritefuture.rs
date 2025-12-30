// Generated macro for WriteFuture (struct)
macro_rules! Depcrate_ioWriteFuture {
() => {
// Module: crate::io
// Provides: {"WriteFuture"}
// Dependencies: {}
# [doc = " Future for the [`AsyncWriteExt::write()`] method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct WriteFuture < 'a , W : Unpin + ? Sized > { writer : & 'a mut W , buf : & 'a [u8] , }
};
}
