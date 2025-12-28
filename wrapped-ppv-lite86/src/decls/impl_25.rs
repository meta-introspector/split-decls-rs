macro_rules! deps {
    () => {
        Store!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl < W : Copy + Store < vec128_storage > , G > Store < vec256_storage > for x2 < W , G > { # [inline (always)] unsafe fn unpack (p : vec256_storage) -> Self { let p = p . split128 () ; x2 :: new ([W :: unpack (p [0]) , W :: unpack (p [1])]) } }
    };
}

impl_25!()