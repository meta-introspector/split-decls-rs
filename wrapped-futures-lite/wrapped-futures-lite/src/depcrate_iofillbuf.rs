// Generated macro for FillBuf (struct)
macro_rules! Depcrate_ioFillBuf {
() => {
// Module: crate::io
// Provides: {"FillBuf"}
// Dependencies: {}
# [doc = " Future for the [`AsyncBufReadExt::fill_buf()`] method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct FillBuf < 'a , R : ? Sized > { reader : Option < & 'a mut R > , }
};
}
