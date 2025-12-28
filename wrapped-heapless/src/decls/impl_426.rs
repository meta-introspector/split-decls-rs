macro_rules! deps {
    () => {
        Sealed!();
        Max!();
    };
}

macro_rules! impl_426 {
    () => {
        deps!();
        impl private :: Sealed for Max { }
    };
}

impl_426!();