macro_rules! deps {
    () => {
        Read!();
        Result!();
    };
}

macro_rules! impl_263 {
    () => {
        deps!();
        impl < R : Read + ? Sized > Read for alloc :: boxed :: Box < R > { # [inline (always)] fn read (& mut self , buf : & mut [u8]) -> crate :: Result < usize > { (* * self) . read (buf) } # [inline (always)] fn read_exact (& mut self , buf : & mut [u8]) -> crate :: Result < () > { (* * self) . read_exact (buf) } }
    };
}

impl_263!()