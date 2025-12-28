macro_rules! deps {
    () => {
        Min!();
        Sealed!();
    };
}

macro_rules! impl_427 {
    () => {
        deps!();
        impl private :: Sealed for Min { }
    };
}

impl_427!()