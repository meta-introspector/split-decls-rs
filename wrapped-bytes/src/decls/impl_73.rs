macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl Deref for Bytes { type Target = [u8] ; # [inline] fn deref (& self) -> & [u8] { self . as_slice () } }
    };
}

impl_73!();