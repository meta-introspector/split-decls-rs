macro_rules! WriteAll {
    () => {
        # [doc = " Future for the [`write_all`](super::AsyncWriteExt::write_all) method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct WriteAll < 'a , W : ? Sized > { writer : & 'a mut W , buf : & 'a [u8] , }
    };
}

WriteAll!()