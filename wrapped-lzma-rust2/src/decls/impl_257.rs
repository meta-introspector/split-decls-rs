macro_rules! deps {
    () => {
        Result!();
        Read!();
    };
}

macro_rules! impl_257 {
    () => {
        deps!();
        impl < R : Read > Read for & mut R { # [inline (always)] fn read (& mut self , buf : & mut [u8]) -> crate :: Result < usize > { (* * self) . read (buf) } # [inline (always)] fn read_exact (& mut self , buf : & mut [u8]) -> crate :: Result < () > { (* * self) . read_exact (buf) } }
    };
}

impl_257!();