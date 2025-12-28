macro_rules! deps {
    () => {
        ShortRead!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl < R : io :: Read > io :: Read for ShortRead < R > { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { let len = self . max_read_len . max (buf . len ()) ; self . delegate . read (& mut buf [.. len]) } }
    };
}

impl_37!()