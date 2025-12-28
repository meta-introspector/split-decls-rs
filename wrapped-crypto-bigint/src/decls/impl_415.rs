macro_rules! deps {
    () => {
        Wrapping!();
    };
}

macro_rules! impl_415 {
    () => {
        deps!();
        impl < T : WrappingMul > Mul < Self > for Wrapping < T > { type Output = Wrapping < T > ; # [inline] fn mul (self , rhs : Self) -> Self :: Output { Wrapping (self . 0 . wrapping_mul (& rhs . 0)) } }
    };
}

impl_415!();