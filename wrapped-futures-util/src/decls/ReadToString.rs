macro_rules! ReadToString {
    () => {
        # [doc = " Future for the [`read_to_string`](super::AsyncReadExt::read_to_string) method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct ReadToString < 'a , R : ? Sized > { reader : & 'a mut R , buf : & 'a mut String , bytes : Vec < u8 > , start_len : usize , }
    };
}

ReadToString!()