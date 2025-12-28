macro_rules! deps {
    () => {
        Read!();
    };
}

macro_rules! impl_585 {
    () => {
        deps!();
        impl < 'de , R > private :: Sealed for & mut R where R : Read < 'de > { }
    };
}

impl_585!();