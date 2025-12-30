// Generated macro for ReadVectoredFuture (struct)
macro_rules! Depcrate_ioReadVectoredFuture {
() => {
// Module: crate::io
// Provides: {"ReadVectoredFuture"}
// Dependencies: {}
# [doc = " Future for the [`AsyncReadExt::read_vectored()`] method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct ReadVectoredFuture < 'a , R : Unpin + ? Sized > { reader : & 'a mut R , bufs : & 'a mut [IoSliceMut < 'a >] , }
};
}
