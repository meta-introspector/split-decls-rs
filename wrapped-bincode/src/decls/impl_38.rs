macro_rules! deps {
    () => {
        VecWriter!();
        Writer!();
        EncodeError!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl enc :: write :: Writer for VecWriter { # [inline (always)] fn write (& mut self , bytes : & [u8]) -> Result < () , EncodeError > { self . inner . extend_from_slice (bytes) ; Ok (()) } }
    };
}

impl_38!();