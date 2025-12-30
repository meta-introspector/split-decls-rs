// Generated macro for WriteAllFuture (struct)
macro_rules! Depcrate_ioWriteAllFuture {
() => {
// Module: crate::io
// Provides: {"WriteAllFuture"}
// Dependencies: {}
# [doc = " Future for the [`AsyncWriteExt::write_all()`] method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct WriteAllFuture < 'a , W : Unpin + ? Sized > { writer : & 'a mut W , buf : & 'a [u8] , }
};
}
