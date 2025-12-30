// Generated macro for WriteAll (struct)
macro_rules! Depcrate_io_write_allWriteAll {
() => {
// Module: crate::io::write_all
// Provides: {"WriteAll"}
// Dependencies: {}
# [doc = " Future for the [`write_all`](super::AsyncWriteExt::write_all) method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct WriteAll < 'a , W : ? Sized > { writer : & 'a mut W , buf : & 'a [u8] , }
};
}
