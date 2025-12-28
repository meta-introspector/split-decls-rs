macro_rules! deps {
    () => {
        Error!();
        Indexer!();
    };
}

macro_rules! impl_406 {
    () => {
        deps!();
        impl io :: Write for Indexer < '_ > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { unsafe { let ptr = buf . as_ptr () as * mut c_void ; let len = buf . len () ; let res = raw :: git_indexer_append (self . raw , ptr , len , & mut self . progress) ; if res < 0 { Err (io :: Error :: new (io :: ErrorKind :: Other , Error :: last_error (res))) } else { Ok (buf . len ()) } } } fn flush (& mut self) -> io :: Result < () > { Ok (()) } }
    };
}

impl_406!()