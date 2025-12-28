macro_rules! deps {
    () => {
        Sealed!();
        Max!();
    };
}

macro_rules! impl_362 {
    () => {
        deps!();
        impl private :: Sealed for Max { }
    };
}

impl_362!()