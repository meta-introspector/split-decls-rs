macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl private :: Sealed for Entry { }
    };
}

impl_23!()