macro_rules! ReadToEnd {
    () => {
        # [doc = " Future for the [`read_to_end`](super::AsyncReadExt::read_to_end) method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct ReadToEnd < 'a , R : ? Sized > { reader : & 'a mut R , buf : & 'a mut Vec < u8 > , start_len : usize , }
    };
}

ReadToEnd!()