macro_rules! retry_interrupted_write_all {
    () => {
        # [doc = " Retry writes until all the data is written or an error that isn't Interrupted is returned."] fn retry_interrupted_write_all < W : Write > (w : & mut W , buf : & [u8]) -> io :: Result < () > { let mut bytes_consumed = 0 ; while bytes_consumed < buf . len () { let res = w . write (& buf [bytes_consumed ..]) ; match res { Ok (len) => bytes_consumed += len , Err (e) => match e . kind () { io :: ErrorKind :: Interrupted => continue , _ => return Err (e) , } , } } Ok (()) }
    };
}

retry_interrupted_write_all!();