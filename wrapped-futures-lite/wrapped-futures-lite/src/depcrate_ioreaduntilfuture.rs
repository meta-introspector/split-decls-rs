// Generated macro for ReadUntilFuture (struct)
macro_rules! Depcrate_ioReadUntilFuture {
() => {
// Module: crate::io
// Provides: {"ReadUntilFuture"}
// Dependencies: {}
# [doc = " Future for the [`AsyncBufReadExt::read_until()`] method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct ReadUntilFuture < 'a , R : Unpin + ? Sized > { reader : & 'a mut R , byte : u8 , buf : & 'a mut Vec < u8 > , read : usize , }
};
}
