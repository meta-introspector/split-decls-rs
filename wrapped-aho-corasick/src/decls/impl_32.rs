macro_rules! impl_32 {
    () => {
        impl < 'a , T : private :: Sealed + ? Sized > private :: Sealed for & 'a T { }
    };
}

impl_32!();