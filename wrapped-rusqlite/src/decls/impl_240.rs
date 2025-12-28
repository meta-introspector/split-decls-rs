macro_rules! deps {
    () => {
        Data!();
        OwnedData!();
        SharedData!();
    };
}

macro_rules! impl_240 {
    () => {
        deps!();
        impl Deref for Data < '_ > { type Target = [u8] ; fn deref (& self) -> & [u8] { let (ptr , sz) = match self { Data :: Owned (OwnedData { ptr , sz }) => (ptr . as_ptr () , * sz) , Data :: Shared (SharedData { ptr , sz , .. }) => (ptr . as_ptr () , * sz) , } ; unsafe { std :: slice :: from_raw_parts (ptr , sz) } } }
    };
}

impl_240!();