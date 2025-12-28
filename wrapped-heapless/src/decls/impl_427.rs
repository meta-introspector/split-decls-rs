macro_rules! deps {
    () => {
        Sealed!();
        Min!();
    };
}

macro_rules! impl_427 {
    () => {
        deps!();
        impl private :: Sealed for Min { }
    };
}

impl_427!();