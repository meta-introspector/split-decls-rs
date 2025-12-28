macro_rules! deps {
    () => {
        Wrapping!();
    };
}

macro_rules! impl_407 {
    () => {
        deps!();
        impl < T : WrappingAdd > Add < Self > for Wrapping < T > { type Output = Wrapping < T > ; # [inline] fn add (self , rhs : Self) -> Self :: Output { Wrapping (self . 0 . wrapping_add (& rhs . 0)) } }
    };
}

impl_407!()