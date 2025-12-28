macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl core :: ops :: Deref for Value { type Target = [u8] ; fn deref (& self) -> & [u8] { & self . data } }
    };
}

impl_68!()