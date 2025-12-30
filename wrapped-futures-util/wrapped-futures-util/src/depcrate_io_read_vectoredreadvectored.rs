// Generated macro for ReadVectored (struct)
macro_rules! Depcrate_io_read_vectoredReadVectored {
() => {
// Module: crate::io::read_vectored
// Provides: {"ReadVectored"}
// Dependencies: {}
# [doc = " Future for the [`read_vectored`](super::AsyncReadExt::read_vectored) method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct ReadVectored < 'a , 'b , R : ? Sized > { reader : & 'a mut R , bufs : & 'a mut [IoSliceMut < 'b >] , }
};
}
