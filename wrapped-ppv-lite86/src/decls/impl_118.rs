macro_rules! deps {
    () => {
        RotateEachWord64!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        impl < S3 : Copy , S4 : Copy , NI : Copy > RotateEachWord64 for u128x1_sse2 < S3 , S4 , NI > { rotr_128 ! (rotate_each_word_right32 , 32) ; }
    };
}

impl_118!();