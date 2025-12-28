macro_rules! deps {
    () => {
        ByteSize!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl < T > ops :: Sub < T > for ByteSize where T : Into < u64 > , { type Output = ByteSize ; # [inline (always)] fn sub (self , rhs : T) -> ByteSize { ByteSize (self . 0 - (rhs . into ())) } }
    };
}

impl_68!()