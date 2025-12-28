macro_rules! deps {
    () => {
        RotateEachWord32!();
        NoS3!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        impl < S4 : Copy , NI : Copy > RotateEachWord32 for u32x4_sse2 < NoS3 , S4 , NI > { rotr_32 ! (rotate_each_word_right7 , 7) ; rotr_32 ! (rotate_each_word_right8 , 8) ; rotr_32 ! (rotate_each_word_right11 , 11) ; rotr_32 ! (rotate_each_word_right12 , 12) ; # [inline (always)] fn rotate_each_word_right16 (self) -> Self { Self :: new (swap16_s2 (self . x)) } rotr_32 ! (rotate_each_word_right20 , 20) ; rotr_32 ! (rotate_each_word_right24 , 24) ; rotr_32 ! (rotate_each_word_right25 , 25) ; }
    };
}

impl_110!()