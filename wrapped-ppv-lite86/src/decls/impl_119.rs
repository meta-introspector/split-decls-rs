macro_rules! deps {
    () => {
        RotateEachWord128!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        impl < S3 : Copy , S4 : Copy , NI : Copy > RotateEachWord128 for u128x1_sse2 < S3 , S4 , NI > { }
    };
}

impl_119!();