// Generated macro for Seek (struct)
macro_rules! Depcrate_io_seekSeek {
() => {
// Module: crate::io::seek
// Provides: {"Seek"}
// Dependencies: {}
# [doc = " Future for the [`seek`](crate::io::AsyncSeekExt::seek) method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Seek < 'a , S : ? Sized > { seek : & 'a mut S , pos : SeekFrom , }
};
}
