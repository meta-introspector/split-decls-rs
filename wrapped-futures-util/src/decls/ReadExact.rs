macro_rules! ReadExact {
    () => {
        # [doc = " Future for the [`read_exact`](super::AsyncReadExt::read_exact) method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct ReadExact < 'a , R : ? Sized > { reader : & 'a mut R , buf : & 'a mut [u8] , }
    };
}

ReadExact!();