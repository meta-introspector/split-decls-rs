macro_rules! deps {
    () => {
        Pointable!();
        Owned!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl < T : ? Sized + Pointable > crate :: sealed :: Sealed for Owned < T > { }
    };
}

impl_34!();