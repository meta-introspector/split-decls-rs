macro_rules! deps {
    () => {
        BytesMut!();
    };
}

macro_rules! impl_187 {
    () => {
        deps!();
        impl Deref for BytesMut { type Target = [u8] ; # [inline] fn deref (& self) -> & [u8] { self . as_ref () } }
    };
}

impl_187!();