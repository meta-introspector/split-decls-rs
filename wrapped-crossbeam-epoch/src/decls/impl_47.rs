macro_rules! deps {
    () => {
        Owned!();
        Pointable!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl < T : ? Sized + Pointable > AsRef < T > for Owned < T > { fn as_ref (& self) -> & T { self . deref () } }
    };
}

impl_47!()