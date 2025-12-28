macro_rules! deps {
    () => {
        SizeWriter!();
        Writer!();
        EncodeError!();
    };
}

macro_rules! impl_492 {
    () => {
        deps!();
        impl Writer for SizeWriter { # [inline (always)] fn write (& mut self , bytes : & [u8]) -> Result < () , EncodeError > { self . bytes_written += bytes . len () ; Ok (()) } }
    };
}

impl_492!()