macro_rules! deps {
    () => {
        Pointable!();
        Owned!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl < T : ? Sized + Pointable > AsMut < T > for Owned < T > { fn as_mut (& mut self) -> & mut T { self . deref_mut () } }
    };
}

impl_48!();