macro_rules! impl_26 {
    () => {
        impl < W , G > From < x2 < W , G > > for vec256_storage where W : Copy , vec128_storage : From < W > , { # [inline (always)] fn from (x : x2 < W , G >) -> Self { vec256_storage :: new128 ([x . 0 [0] . into () , x . 0 [1] . into ()]) } }
    };
}

impl_26!()