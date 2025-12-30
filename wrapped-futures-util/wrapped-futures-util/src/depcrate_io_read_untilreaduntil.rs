// Generated macro for ReadUntil (struct)
macro_rules! Depcrate_io_read_untilReadUntil {
() => {
// Module: crate::io::read_until
// Provides: {"ReadUntil"}
// Dependencies: {}
# [doc = " Future for the [`read_until`](super::AsyncBufReadExt::read_until) method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct ReadUntil < 'a , R : ? Sized > { reader : & 'a mut R , byte : u8 , buf : & 'a mut Vec < u8 > , read : usize , }
};
}
