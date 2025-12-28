macro_rules! deps {
    () => {
        RotateEachWord32!();
        YesS3!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        impl < S4 : Copy , NI : Copy > RotateEachWord32 for u32x4_sse2 < YesS3 , S4 , NI > { rotr_32 ! (rotate_each_word_right7 , 7) ; rotr_32_s3 ! (rotate_each_word_right8 , 0x0c0f_0e0d_080b_0a09 , 0x0407_0605_0003_0201) ; rotr_32 ! (rotate_each_word_right11 , 11) ; rotr_32 ! (rotate_each_word_right12 , 12) ; rotr_32_s3 ! (rotate_each_word_right16 , 0x0d0c_0f0e_0908_0b0a , 0x0504_0706_0100_0302) ; rotr_32 ! (rotate_each_word_right20 , 20) ; rotr_32_s3 ! (rotate_each_word_right24 , 0x0e0d_0c0f_0a09_080b , 0x0605_0407_0201_0003) ; rotr_32 ! (rotate_each_word_right25 , 25) ; }
    };
}

impl_109!()