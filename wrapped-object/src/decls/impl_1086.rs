macro_rules! deps {
    () => {
        ByteString!();
    };
}

macro_rules! impl_1086 {
    () => {
        deps!();
        impl < 'a > core :: ops :: Deref for ByteString < 'a > { type Target = [u8] ; fn deref (& self) -> & [u8] { self . 0 . deref () } }
    };
}

impl_1086!()