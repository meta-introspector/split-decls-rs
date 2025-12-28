macro_rules! deps {
    () => {
        Owned!();
        Pointable!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl < T : ? Sized + Pointable > Borrow < T > for Owned < T > { fn borrow (& self) -> & T { self . deref () } }
    };
}

impl_45!();