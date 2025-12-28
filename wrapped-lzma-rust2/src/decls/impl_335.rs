macro_rules! deps {
    () => {
        Result!();
        Read!();
        CountingReader!();
    };
}

macro_rules! impl_335 {
    () => {
        deps!();
        impl < R : Read > Read for CountingReader < R > { fn read (& mut self , buf : & mut [u8]) -> Result < usize > { let read_size = self . inner . read (buf) ? ; self . bytes_read += read_size as u64 ; Ok (read_size) } }
    };
}

impl_335!()