macro_rules! deps {
    () => {
        PassThrough!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl < R > io :: Read for PassThrough < R > where R : io :: Read , { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { let bytes_read = self . reader . read (buf) ? ; if let Some (writer) = self . writer . as_mut () { use std :: io :: Write ; writer . lock () . write_all (& buf [.. bytes_read]) ? ; } Ok (bytes_read) } }
    };
}

impl_13!()