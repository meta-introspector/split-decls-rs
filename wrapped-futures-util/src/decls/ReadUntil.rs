macro_rules! ReadUntil {
    () => {
        # [doc = " Future for the [`read_until`](super::AsyncBufReadExt::read_until) method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct ReadUntil < 'a , R : ? Sized > { reader : & 'a mut R , byte : u8 , buf : & 'a mut Vec < u8 > , read : usize , }
    };
}

ReadUntil!()