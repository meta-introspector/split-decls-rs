// Generated macro for SeekFuture (struct)
macro_rules! Depcrate_ioSeekFuture {
() => {
// Module: crate::io
// Provides: {"SeekFuture"}
// Dependencies: {}
# [doc = " Future for the [`AsyncSeekExt::seek()`] method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct SeekFuture < 'a , S : Unpin + ? Sized > { seeker : & 'a mut S , pos : SeekFrom , }
};
}
