macro_rules! deps {
    () => {
        Error!();
        OdbWriter!();
    };
}

macro_rules! impl_506 {
    () => {
        deps!();
        impl < 'repo > io :: Write for OdbWriter < 'repo > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { unsafe { let ptr = buf . as_ptr () as * const c_char ; let len = buf . len () ; let res = raw :: git_odb_stream_write (self . raw , ptr , len) ; if res < 0 { Err (io :: Error :: new (io :: ErrorKind :: Other , "Write error")) } else { Ok (buf . len ()) } } } fn flush (& mut self) -> io :: Result < () > { Ok (()) } }
    };
}

impl_506!()