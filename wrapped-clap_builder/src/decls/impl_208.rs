macro_rules! deps {
    () => {
        Str!();
    };
}

macro_rules! impl_208 {
    () => {
        deps!();
        impl AsRef < [u8] > for Str { # [inline] fn as_ref (& self) -> & [u8] { self . as_bytes () } }
    };
}

impl_208!()