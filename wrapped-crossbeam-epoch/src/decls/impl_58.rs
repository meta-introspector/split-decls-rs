macro_rules! deps {
    () => {
        Shared!();
        Pointable!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl < T : ? Sized + Pointable > Eq for Shared < '_ , T > { }
    };
}

impl_58!()