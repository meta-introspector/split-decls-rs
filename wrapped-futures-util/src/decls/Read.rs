macro_rules! Read {
    () => {
        # [doc = " Future for the [`read`](super::AsyncReadExt::read) method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Read < 'a , R : ? Sized > { reader : & 'a mut R , buf : & 'a mut [u8] , }
    };
}

Read!();