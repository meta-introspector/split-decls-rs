macro_rules! deps {
    () => {
        MmapMut!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl AsMut < [u8] > for MmapMut { # [inline] fn as_mut (& mut self) -> & mut [u8] { self . deref_mut () } }
    };
}

impl_30!()