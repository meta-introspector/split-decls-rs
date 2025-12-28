macro_rules! deps {
    () => {
        Result!();
        Write!();
    };
}

macro_rules! impl_260 {
    () => {
        deps!();
        impl < W : Write > Write for & mut W { # [inline (always)] fn write (& mut self , buf : & [u8]) -> crate :: Result < usize > { (* * self) . write (buf) } # [inline (always)] fn flush (& mut self) -> crate :: Result < () > { (* * self) . flush () } }
    };
}

impl_260!();