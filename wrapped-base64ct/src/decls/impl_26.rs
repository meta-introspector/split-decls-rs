macro_rules! deps {
    () => {
        Encoding!();
        Encoder!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl < E : Encoding > io :: Write for Encoder < '_ , E > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . encode (buf) ? ; Ok (buf . len ()) } fn flush (& mut self) -> io :: Result < () > { Ok (()) } }
    };
}

impl_26!()