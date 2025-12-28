macro_rules! deps {
    () => {
        Blob!();
    };
}

macro_rules! impl_417 {
    () => {
        deps!();
        impl std :: ops :: Deref for Blob { type Target = [u8] ; fn deref (& self) -> & [u8] { self . slice } }
    };
}

impl_417!();