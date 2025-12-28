macro_rules! deps {
    () => {
        BytesMut!();
    };
}

macro_rules! impl_189 {
    () => {
        deps!();
        impl DerefMut for BytesMut { # [inline] fn deref_mut (& mut self) -> & mut [u8] { self . as_mut () } }
    };
}

impl_189!()