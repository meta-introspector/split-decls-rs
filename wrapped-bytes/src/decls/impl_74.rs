macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        impl AsRef < [u8] > for Bytes { # [inline] fn as_ref (& self) -> & [u8] { self . as_slice () } }
    };
}

impl_74!()