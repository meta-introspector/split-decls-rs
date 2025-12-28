macro_rules! FillBuf {
    () => {
        # [doc = " Future for the [`fill_buf`](super::AsyncBufReadExt::fill_buf) method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct FillBuf < 'a , R : ? Sized > { reader : Option < & 'a mut R > , }
    };
}

FillBuf!()