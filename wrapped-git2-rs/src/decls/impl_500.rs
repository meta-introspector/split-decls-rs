macro_rules! deps {
    () => {
        OdbReader!();
        Error!();
    };
}

macro_rules! impl_500 {
    () => {
        deps!();
        impl < 'repo > io :: Read for OdbReader < 'repo > { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { unsafe { let ptr = buf . as_ptr () as * mut c_char ; let len = buf . len () ; let res = raw :: git_odb_stream_read (self . raw , ptr , len) ; if res < 0 { Err (io :: Error :: new (io :: ErrorKind :: Other , "Read error")) } else { Ok (res as _) } } } }
    };
}

impl_500!();