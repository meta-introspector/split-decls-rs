macro_rules! deps {
    () => {
        Write!();
        Result!();
    };
}

macro_rules! impl_262 {
    () => {
        deps!();
        impl Write for Vec < u8 > { # [inline (always)] fn write (& mut self , buf : & [u8]) -> crate :: Result < usize > { self . extend_from_slice (buf) ; Ok (buf . len ()) } # [inline (always)] fn flush (& mut self) -> crate :: Result < () > { Ok (()) } }
    };
}

impl_262!();