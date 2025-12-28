macro_rules! deps {
    () => {
        WriteWithProgress!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl < T > std :: io :: Write for WriteWithProgress < '_ , T > where T : std :: io :: Write , { fn write (& mut self , buf : & [u8]) -> std :: io :: Result < usize > { let written = self . inner . write (buf) ? ; self . progress . fetch_add (written as gix_features :: progress :: Step , Ordering :: SeqCst) ; Ok (written) } fn flush (& mut self) -> std :: io :: Result < () > { self . inner . flush () } }
    };
}

impl_13!();