macro_rules! deps {
    () => {
        MmapMut!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl AsRef < [u8] > for MmapMut { # [inline] fn as_ref (& self) -> & [u8] { self . deref () } }
    };
}

impl_34!();