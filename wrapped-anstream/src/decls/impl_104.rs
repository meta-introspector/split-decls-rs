macro_rules! deps {
    () => {
        Buffer!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl AsRef < [u8] > for Buffer { # [inline] fn as_ref (& self) -> & [u8] { self . as_bytes () } }
    };
}

impl_104!();