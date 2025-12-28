macro_rules! ReadLine {
    () => {
        # [doc = " Future for the [`read_line`](super::AsyncBufReadExt::read_line) method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct ReadLine < 'a , R : ? Sized > { reader : & 'a mut R , buf : & 'a mut String , bytes : Vec < u8 > , read : usize , finished : bool , }
    };
}

ReadLine!();