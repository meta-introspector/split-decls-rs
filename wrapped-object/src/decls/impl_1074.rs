macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! impl_1074 {
    () => {
        deps!();
        impl < 'a > core :: ops :: Deref for Bytes < 'a > { type Target = [u8] ; fn deref (& self) -> & [u8] { self . 0 . deref () } }
    };
}

impl_1074!()