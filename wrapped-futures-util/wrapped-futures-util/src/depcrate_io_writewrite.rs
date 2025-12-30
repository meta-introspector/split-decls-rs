// Generated macro for Write (struct)
macro_rules! Depcrate_io_writeWrite {
() => {
// Module: crate::io::write
// Provides: {"Write"}
// Dependencies: {}
# [doc = " Future for the [`write`](super::AsyncWriteExt::write) method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Write < 'a , W : ? Sized > { writer : & 'a mut W , buf : & 'a [u8] , }
};
}
