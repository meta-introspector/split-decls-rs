macro_rules! deps {
    () => {
        Wrapping!();
    };
}

macro_rules! impl_411 {
    () => {
        deps!();
        impl < T : WrappingSub > Sub < Self > for Wrapping < T > { type Output = Wrapping < T > ; # [inline] fn sub (self , rhs : Self) -> Self :: Output { Wrapping (self . 0 . wrapping_sub (& rhs . 0)) } }
    };
}

impl_411!();