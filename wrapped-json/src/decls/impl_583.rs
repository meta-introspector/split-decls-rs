macro_rules! deps {
    () => {
        StrRead!();
    };
}

macro_rules! impl_583 {
    () => {
        deps!();
        impl < 'a > private :: Sealed for StrRead < 'a > { }
    };
}

impl_583!();