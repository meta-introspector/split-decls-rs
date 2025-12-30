// Generated macro for ReadToStringFuture (struct)
macro_rules! Depcrate_ioReadToStringFuture {
() => {
// Module: crate::io
// Provides: {"ReadToStringFuture"}
// Dependencies: {}
# [doc = " Future for the [`AsyncReadExt::read_to_string()`] method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct ReadToStringFuture < 'a , R : Unpin + ? Sized > { reader : & 'a mut R , buf : & 'a mut String , bytes : Vec < u8 > , start_len : usize , }
};
}
