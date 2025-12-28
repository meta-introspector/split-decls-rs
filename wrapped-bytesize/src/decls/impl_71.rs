macro_rules! deps {
    () => {
        ByteSize!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl < T > ops :: MulAssign < T > for ByteSize where T : Into < u64 > , { # [inline (always)] fn mul_assign (& mut self , rhs : T) { self . 0 *= rhs . into () ; } }
    };
}

impl_71!()