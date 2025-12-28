macro_rules! deps {
    () => {
        ByteSize!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl ops :: Add < ByteSize > for ByteSize { type Output = ByteSize ; # [inline (always)] fn add (self , rhs : ByteSize) -> ByteSize { ByteSize (self . 0 + rhs . 0) } }
    };
}

impl_62!()