macro_rules! deps {
    () => {
        Wrapping!();
    };
}

macro_rules! impl_420 {
    () => {
        deps!();
        impl < T : WrappingNeg > Neg for & Wrapping < T > { type Output = Wrapping < T > ; # [inline] fn neg (self) -> Self :: Output { Wrapping (self . 0 . wrapping_neg ()) } }
    };
}

impl_420!()