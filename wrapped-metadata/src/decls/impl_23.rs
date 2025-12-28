macro_rules! deps {
    () => {
        Blob!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl std :: ops :: Deref for Blob < '_ > { type Target = [u8] ; fn deref (& self) -> & [u8] { self . slice } }
    };
}

impl_23!();