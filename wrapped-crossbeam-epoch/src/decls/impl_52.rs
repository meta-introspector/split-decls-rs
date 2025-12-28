macro_rules! deps {
    () => {
        Shared!();
        Pointable!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl < T : ? Sized + Pointable > crate :: sealed :: Sealed for Shared < '_ , T > { }
    };
}

impl_52!();