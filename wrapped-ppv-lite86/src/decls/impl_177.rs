macro_rules! deps {
    () => {
        Vector!();
    };
}

macro_rules! impl_177 {
    () => {
        deps!();
        impl < S3 , S4 , NI > Vector < [u32 ; 16] > for u32x4x4_sse2 < S3 , S4 , NI > { # [inline (always)] fn to_scalars (self) -> [u32 ; 16] { transmute ! (self) } }
    };
}

impl_177!()