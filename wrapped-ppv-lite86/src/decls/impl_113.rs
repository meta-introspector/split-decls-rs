macro_rules! deps {
    () => {
        YesS3!();
        RotateEachWord32!();
    };
}

macro_rules! impl_113 {
    () => {
        deps!();
        impl < S4 : Copy , NI : Copy > RotateEachWord32 for u64x2_sse2 < YesS3 , S4 , NI > { rotr_64 ! (rotate_each_word_right7 , 7) ; rotr_64_s3 ! (rotate_each_word_right8 , 0x080f_0e0d_0c0b_0a09 , 0x0007_0605_0403_0201) ; rotr_64 ! (rotate_each_word_right11 , 11) ; rotr_64 ! (rotate_each_word_right12 , 12) ; rotr_64_s3 ! (rotate_each_word_right16 , 0x0908_0f0e_0d0c_0b0a , 0x0100_0706_0504_0302) ; rotr_64 ! (rotate_each_word_right20 , 20) ; rotr_64_s3 ! (rotate_each_word_right24 , 0x0a09_080f_0e0d_0c0b , 0x0201_0007_0605_0403) ; rotr_64 ! (rotate_each_word_right25 , 25) ; }
    };
}

impl_113!()