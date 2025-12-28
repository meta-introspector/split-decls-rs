macro_rules! deps {
    () => {
        Literal!();
    };
}

macro_rules! impl_169 {
    () => {
        deps!();
        impl AsRef < [u8] > for Literal { fn as_ref (& self) -> & [u8] { self . as_bytes () } }
    };
}

impl_169!();