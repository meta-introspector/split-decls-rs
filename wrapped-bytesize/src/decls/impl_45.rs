macro_rules! deps {
    () => {
        ByteSize!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl < T > ops :: AddAssign < T > for ByteSize where T : Into < u64 > , { # [inline (always)] fn add_assign (& mut self , rhs : T) { self . 0 += rhs . into () ; } }
    };
}

impl_45!()