// Generated macro for ReadFuture (struct)
macro_rules! Depcrate_ioReadFuture {
() => {
// Module: crate::io
// Provides: {"ReadFuture"}
// Dependencies: {}
# [doc = " Future for the [`AsyncReadExt::read()`] method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct ReadFuture < 'a , R : Unpin + ? Sized > { reader : & 'a mut R , buf : & 'a mut [u8] , }
};
}
