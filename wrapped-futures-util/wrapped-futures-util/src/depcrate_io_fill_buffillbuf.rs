// Generated macro for FillBuf (struct)
macro_rules! Depcrate_io_fill_bufFillBuf {
() => {
// Module: crate::io::fill_buf
// Provides: {"FillBuf"}
// Dependencies: {}
# [doc = " Future for the [`fill_buf`](super::AsyncBufReadExt::fill_buf) method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct FillBuf < 'a , R : ? Sized > { reader : Option < & 'a mut R > , }
};
}
