macro_rules! deps {
    () => {
        Wrapping!();
    };
}

macro_rules! impl_424 {
    () => {
        deps!();
        impl < T : WrappingShr > Shr < u32 > for & Wrapping < T > { type Output = Wrapping < T > ; # [inline] fn shr (self , rhs : u32) -> Self :: Output { Wrapping (self . 0 . wrapping_shr (rhs)) } }
    };
}

impl_424!()