// Generated macro for ReadToEnd (struct)
macro_rules! Depcrate_io_read_to_endReadToEnd {
() => {
// Module: crate::io::read_to_end
// Provides: {"ReadToEnd"}
// Dependencies: {}
# [doc = " Future for the [`read_to_end`](super::AsyncReadExt::read_to_end) method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct ReadToEnd < 'a , R : ? Sized > { reader : & 'a mut R , buf : & 'a mut Vec < u8 > , start_len : usize , }
};
}
