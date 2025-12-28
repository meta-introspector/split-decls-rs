macro_rules! deps {
    () => {
        Wrapping!();
    };
}

macro_rules! impl_410 {
    () => {
        deps!();
        impl < T : WrappingAdd > Add < & Wrapping < T > > for & Wrapping < T > { type Output = Wrapping < T > ; # [inline] fn add (self , rhs : & Wrapping < T >) -> Self :: Output { Wrapping (self . 0 . wrapping_add (& rhs . 0)) } }
    };
}

impl_410!();