macro_rules! ReadExactFuture {
    () => {
        # [doc = " Future for the [`AsyncReadExt::read_exact()`] method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct ReadExactFuture < 'a , R : Unpin + ? Sized > { reader : & 'a mut R , buf : & 'a mut [u8] , }
    };
}

ReadExactFuture!()