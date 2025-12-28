macro_rules! deps {
    () => {
        PassThrough!();
    };
}

macro_rules! impl_149 {
    () => {
        deps!();
        impl < R , W > io :: Read for PassThrough < R , W > where W : io :: Write , R : io :: Read , { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { let bytes_read = self . read . read (buf) ? ; self . write . write_all (& buf [.. bytes_read]) ? ; Ok (bytes_read) } }
    };
}

impl_149!();