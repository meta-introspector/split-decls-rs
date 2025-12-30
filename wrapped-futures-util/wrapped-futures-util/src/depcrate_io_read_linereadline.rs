// Generated macro for ReadLine (struct)
macro_rules! Depcrate_io_read_lineReadLine {
() => {
// Module: crate::io::read_line
// Provides: {"ReadLine"}
// Dependencies: {}
# [doc = " Future for the [`read_line`](super::AsyncBufReadExt::read_line) method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct ReadLine < 'a , R : ? Sized > { reader : & 'a mut R , buf : & 'a mut String , bytes : Vec < u8 > , read : usize , finished : bool , }
};
}
