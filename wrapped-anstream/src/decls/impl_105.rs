macro_rules! deps {
    () => {
        Buffer!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        impl std :: io :: Write for Buffer { # [inline] fn write (& mut self , buf : & [u8]) -> std :: io :: Result < usize > { self . 0 . extend (buf) ; Ok (buf . len ()) } # [inline] fn flush (& mut self) -> std :: io :: Result < () > { Ok (()) } }
    };
}

impl_105!();