macro_rules! deps {
    () => {
        BytesMut!();
    };
}

macro_rules! impl_188 {
    () => {
        deps!();
        impl AsMut < [u8] > for BytesMut { # [inline] fn as_mut (& mut self) -> & mut [u8] { self . as_slice_mut () } }
    };
}

impl_188!()