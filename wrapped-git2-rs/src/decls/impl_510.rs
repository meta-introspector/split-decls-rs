macro_rules! deps {
    () => {
        Error!();
        OdbPackwriter!();
    };
}

macro_rules! impl_510 {
    () => {
        deps!();
        impl < 'repo > io :: Write for OdbPackwriter < 'repo > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { unsafe { let ptr = buf . as_ptr () as * mut c_void ; let len = buf . len () ; let writepack = & * self . raw ; let res = match writepack . append { Some (append) => append (self . raw , ptr , len , & mut self . progress) , None => - 1 , } ; if res < 0 { Err (io :: Error :: new (io :: ErrorKind :: Other , "Write error")) } else { Ok (buf . len ()) } } } fn flush (& mut self) -> io :: Result < () > { Ok (()) } }
    };
}

impl_510!()