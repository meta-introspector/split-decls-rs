// Generated macro for Close (struct)
macro_rules! Depcrate_io_closeClose {
() => {
// Module: crate::io::close
// Provides: {"Close"}
// Dependencies: {}
# [doc = " Future for the [`close`](super::AsyncWriteExt::close) method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Close < 'a , W : ? Sized > { writer : & 'a mut W , }
};
}
