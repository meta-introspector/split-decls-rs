macro_rules! impl_198 {
    () => {
        impl < S3 , S4 , NI > PartialEq for u32x4_sse2 < S3 , S4 , NI > { # [inline (always)] fn eq (& self , rhs : & Self) -> bool { unsafe { eq128_s2 (self . x , rhs . x) } } }
    };
}

impl_198!()