macro_rules! deps {
    () => {
        Pointable!();
        Owned!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl < T : ? Sized + Pointable > AsRef < T > for Owned < T > { fn as_ref (& self) -> & T { self . deref () } }
    };
}

impl_47!();