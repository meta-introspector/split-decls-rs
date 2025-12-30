// Generated macro for ReadToEndFuture (struct)
macro_rules! Depcrate_ioReadToEndFuture {
() => {
// Module: crate::io
// Provides: {"ReadToEndFuture"}
// Dependencies: {}
# [doc = " Future for the [`AsyncReadExt::read_to_end()`] method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct ReadToEndFuture < 'a , R : Unpin + ? Sized > { reader : & 'a mut R , buf : & 'a mut Vec < u8 > , start_len : usize , }
};
}
