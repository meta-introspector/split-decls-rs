macro_rules! FlushFuture {
    () => {
        # [doc = " Future for the [`AsyncWriteExt::flush()`] method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct FlushFuture < 'a , W : Unpin + ? Sized > { writer : & 'a mut W , }
    };
}

FlushFuture!();