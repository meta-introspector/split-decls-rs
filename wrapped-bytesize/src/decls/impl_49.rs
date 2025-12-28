macro_rules! deps {
    () => {
        ByteSize!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl < T > ops :: SubAssign < T > for ByteSize where T : Into < u64 > , { # [inline (always)] fn sub_assign (& mut self , rhs : T) { self . 0 -= rhs . into () ; } }
    };
}

impl_49!()