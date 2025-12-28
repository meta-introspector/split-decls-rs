macro_rules! Write {
    () => {
        # [doc = " Future for the [`write`](super::AsyncWriteExt::write) method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Write < 'a , W : ? Sized > { writer : & 'a mut W , buf : & 'a [u8] , }
    };
}

Write!();