macro_rules! deps {
    () => {
        Min!();
        Sealed!();
    };
}

macro_rules! impl_363 {
    () => {
        deps!();
        impl private :: Sealed for Min { }
    };
}

impl_363!();