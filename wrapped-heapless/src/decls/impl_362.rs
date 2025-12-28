macro_rules! deps {
    () => {
        Max!();
        Sealed!();
    };
}

macro_rules! impl_362 {
    () => {
        deps!();
        impl private :: Sealed for Max { }
    };
}

impl_362!();