macro_rules! deps {
    () => {
        TimeBuf!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl std :: io :: Write for TimeBuf { fn write (& mut self , buf : & [u8]) -> std :: io :: Result < usize > { self . buf . extend_from_slice (buf) ; Ok (buf . len ()) } fn flush (& mut self) -> std :: io :: Result < () > { Ok (()) } }
    };
}

impl_26!();