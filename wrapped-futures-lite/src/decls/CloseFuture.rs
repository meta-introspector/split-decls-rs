macro_rules! CloseFuture {
    () => {
        # [doc = " Future for the [`AsyncWriteExt::close()`] method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct CloseFuture < 'a , W : Unpin + ? Sized > { writer : & 'a mut W , }
    };
}

CloseFuture!();