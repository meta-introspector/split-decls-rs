macro_rules! impl_59 {
    () => {
        impl < W > From < x4 < W > > for vec512_storage where W : Copy , vec128_storage : From < W > , { # [inline (always)] fn from (x : x4 < W >) -> Self { vec512_storage :: new128 ([x . 0 [0] . into () , x . 0 [1] . into () , x . 0 [2] . into () , x . 0 [3] . into ()]) } }
    };
}

impl_59!();