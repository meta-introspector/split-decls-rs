macro_rules! deps {
    () => {
        Wrapping!();
    };
}

macro_rules! impl_417 {
    () => {
        deps!();
        impl < T : WrappingMul > Mul < Wrapping < T > > for & Wrapping < T > { type Output = Wrapping < T > ; # [inline] fn mul (self , rhs : Wrapping < T >) -> Self :: Output { Wrapping (self . 0 . wrapping_mul (& rhs . 0)) } }
    };
}

impl_417!()