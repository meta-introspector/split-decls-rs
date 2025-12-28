macro_rules! deps {
    () => {
        ByteSize!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl ops :: Sub < ByteSize > for ByteSize { type Output = ByteSize ; # [inline (always)] fn sub (self , rhs : ByteSize) -> ByteSize { ByteSize (self . 0 - rhs . 0) } }
    };
}

impl_66!()