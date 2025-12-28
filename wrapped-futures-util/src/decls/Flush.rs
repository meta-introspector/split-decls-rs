macro_rules! Flush {
    () => {
        # [doc = " Future for the [`flush`](super::AsyncWriteExt::flush) method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Flush < 'a , W : ? Sized > { writer : & 'a mut W , }
    };
}

Flush!();