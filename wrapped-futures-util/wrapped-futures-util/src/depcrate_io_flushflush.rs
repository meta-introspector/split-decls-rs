// Generated macro for Flush (struct)
macro_rules! Depcrate_io_flushFlush {
() => {
// Module: crate::io::flush
// Provides: {"Flush"}
// Dependencies: {}
# [doc = " Future for the [`flush`](super::AsyncWriteExt::flush) method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Flush < 'a , W : ? Sized > { writer : & 'a mut W , }
};
}
