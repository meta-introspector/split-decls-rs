macro_rules! undiagonalize {
    () => {
        # [inline (always)] fn undiagonalize (row0 : & mut v128 , row2 : & mut v128 , row3 : & mut v128) { * row0 = shuffle_epi32 :: < 0 , 3 , 2 , 1 > (* row0) ; * row3 = shuffle_epi32 :: < 1 , 0 , 3 , 2 > (* row3) ; * row2 = shuffle_epi32 :: < 2 , 1 , 0 , 3 > (* row2) ; }
    };
}

undiagonalize!()