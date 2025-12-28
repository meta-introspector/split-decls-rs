macro_rules! deps {
    () => {
        Object!();
    };
}

macro_rules! impl_168 {
    () => {
        deps!();
        impl AsRef < [u8] > for Object < '_ > { fn as_ref (& self) -> & [u8] { & self . data } }
    };
}

impl_168!()