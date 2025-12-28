macro_rules! WriteVectoredFuture {
    () => {
        # [doc = " Future for the [`AsyncWriteExt::write_vectored()`] method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct WriteVectoredFuture < 'a , W : Unpin + ? Sized > { writer : & 'a mut W , bufs : & 'a [IoSlice < 'a >] , }
    };
}

WriteVectoredFuture!();