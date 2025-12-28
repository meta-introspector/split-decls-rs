macro_rules! deps {
    () => {
        Context!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl core :: io :: Write for Context { # [inline] fn write (& mut self , data : & [u8]) -> core :: io :: Result < usize > { self . consume (data) ; Ok (data . len ()) } # [inline] fn flush (& mut self) -> core :: io :: Result < () > { Ok (()) } }
    };
}

impl_13!();