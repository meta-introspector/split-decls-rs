macro_rules! deps {
    () => {
        Error!();
        BlobWriter!();
    };
}

macro_rules! impl_225 {
    () => {
        deps!();
        impl < 'repo > io :: Write for BlobWriter < 'repo > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { unsafe { if let Some (f) = (* self . raw) . write { let res = f (self . raw , buf . as_ptr () as * const _ , buf . len ()) ; if res < 0 { Err (io :: Error :: new (io :: ErrorKind :: Other , "Write error")) } else { Ok (buf . len ()) } } else { Err (io :: Error :: new (io :: ErrorKind :: Other , "no write callback")) } } } fn flush (& mut self) -> io :: Result < () > { Ok (()) } }
    };
}

impl_225!();