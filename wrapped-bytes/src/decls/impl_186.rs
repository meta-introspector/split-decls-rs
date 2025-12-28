macro_rules! deps {
    () => {
        BytesMut!();
    };
}

macro_rules! impl_186 {
    () => {
        deps!();
        impl AsRef < [u8] > for BytesMut { # [inline] fn as_ref (& self) -> & [u8] { self . as_slice () } }
    };
}

impl_186!();