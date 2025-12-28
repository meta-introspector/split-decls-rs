macro_rules! deps {
    () => {
        Max!();
        Sealed!();
    };
}

macro_rules! impl_426 {
    () => {
        deps!();
        impl private :: Sealed for Max { }
    };
}

impl_426!()