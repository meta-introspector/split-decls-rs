macro_rules! deps {
    () => {
        Wrapping!();
    };
}

macro_rules! impl_414 {
    () => {
        deps!();
        impl < T : WrappingSub > Sub < & Wrapping < T > > for & Wrapping < T > { type Output = Wrapping < T > ; # [inline] fn sub (self , rhs : & Wrapping < T >) -> Self :: Output { Wrapping (self . 0 . wrapping_sub (& rhs . 0)) } }
    };
}

impl_414!()