macro_rules! deps {
    () => {
        Wrapping!();
    };
}

macro_rules! impl_421 {
    () => {
        deps!();
        impl < T : WrappingShl > Shl < u32 > for Wrapping < T > { type Output = Wrapping < T > ; # [inline] fn shl (self , rhs : u32) -> Self :: Output { Wrapping (self . 0 . wrapping_shl (rhs)) } }
    };
}

impl_421!();