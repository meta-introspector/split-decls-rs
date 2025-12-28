macro_rules! deps {
    () => {
        ByteSize!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl < T > ops :: Mul < T > for ByteSize where T : Into < u64 > , { type Output = ByteSize ; # [inline (always)] fn mul (self , rhs : T) -> ByteSize { ByteSize (self . 0 * rhs . into ()) } }
    };
}

impl_50!()