macro_rules! ReadLineFuture {
    () => {
        # [doc = " Future for the [`AsyncBufReadExt::read_line()`] method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct ReadLineFuture < 'a , R : Unpin + ? Sized > { reader : & 'a mut R , buf : & 'a mut String , bytes : Vec < u8 > , read : usize , }
    };
}

ReadLineFuture!()