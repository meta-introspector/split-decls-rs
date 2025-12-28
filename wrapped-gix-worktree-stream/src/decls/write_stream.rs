macro_rules! write_stream {
    () => {
        # [doc = " This writes everything in `input` in such way that the receiver knows exactly how much to read."] # [doc = " The format is similar to the packetline format, but in binary."] pub (crate) fn write_stream (buf : & mut Vec < u8 > , mut input : impl std :: io :: Read , out : & mut gix_features :: io :: pipe :: Writer ,) -> std :: io :: Result < () > { const BUF_LEN : usize = u16 :: MAX as usize ; clear_and_set_len (buf , BUF_LEN) ? ; # [allow (clippy :: unused_io_amount)] loop { match input . read (buf) { Ok (0) => { out . write (& 0_u16 . to_le_bytes ()) ? ; break ; } Ok (n) => { out . write (& (n as u16) . to_le_bytes ()) ? ; out . write (& buf [.. n]) ? ; } Err (ref e) if e . kind () == ErrorKind :: Interrupted => { } Err (e) => return Err (e) , } } Ok (()) }
    };
}

write_stream!()