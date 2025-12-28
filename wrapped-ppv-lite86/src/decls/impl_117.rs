macro_rules! deps {
    () => {
        RotateEachWord32!();
    };
}

macro_rules! impl_117 {
    () => {
        deps!();
        impl < S3 : Copy , S4 : Copy , NI : Copy > RotateEachWord32 for u128x1_sse2 < S3 , S4 , NI > { rotr_128 ! (rotate_each_word_right7 , 7) ; rotr_128 ! (rotate_each_word_right8 , 8) ; rotr_128 ! (rotate_each_word_right11 , 11) ; rotr_128 ! (rotate_each_word_right12 , 12) ; rotr_128 ! (rotate_each_word_right16 , 16) ; rotr_128 ! (rotate_each_word_right20 , 20) ; rotr_128 ! (rotate_each_word_right24 , 24) ; rotr_128 ! (rotate_each_word_right25 , 25) ; }
    };
}

impl_117!()