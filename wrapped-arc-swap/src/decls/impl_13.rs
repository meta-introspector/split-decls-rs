macro_rules! deps {
    () => {
        DynGuard!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl < T : ? Sized > Deref for DynGuard < T > { type Target = T ; fn deref (& self) -> & T { & self . 0 } }
    };
}

impl_13!();