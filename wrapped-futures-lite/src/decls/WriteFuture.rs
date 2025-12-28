macro_rules! WriteFuture {
    () => {
        # [doc = " Future for the [`AsyncWriteExt::write()`] method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct WriteFuture < 'a , W : Unpin + ? Sized > { writer : & 'a mut W , buf : & 'a [u8] , }
    };
}

WriteFuture!();