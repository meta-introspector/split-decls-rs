macro_rules! WriteVectored {
    () => {
        # [doc = " Future for the [`write_vectored`](super::AsyncWriteExt::write_vectored) method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct WriteVectored < 'a , 'b , W : ? Sized > { writer : & 'a mut W , bufs : & 'a [IoSlice < 'b >] , }
    };
}

WriteVectored!();