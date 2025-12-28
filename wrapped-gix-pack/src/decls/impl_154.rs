macro_rules! deps {
    () => {
        HashWrite!();
    };
}

macro_rules! impl_154 {
    () => {
        deps!();
        impl < T > std :: io :: Write for HashWrite < '_ , T > where T : std :: io :: Write , { fn write (& mut self , buf : & [u8]) -> std :: io :: Result < usize > { let written = self . inner . write (buf) ? ; self . hash . update (& buf [.. written]) ; Ok (written) } fn flush (& mut self) -> std :: io :: Result < () > { self . inner . flush () } }
    };
}

impl_154!();