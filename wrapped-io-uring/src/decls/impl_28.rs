macro_rules! deps {
    () => {
        Entry32!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl private :: Sealed for Entry32 { }
    };
}

impl_28!()