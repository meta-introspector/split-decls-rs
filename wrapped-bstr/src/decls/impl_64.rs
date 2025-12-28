macro_rules! impl_64 {
    () => {
        impl < const N : usize > private :: Sealed for [u8 ; N] { }
    };
}

impl_64!()