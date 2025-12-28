macro_rules! transpose_vecs {
    () => {
        # [inline (always)] fn transpose_vecs (vecs : & mut [v128 ; DEGREE]) { let ab_01 = unpacklo_epi32 (vecs [0] , vecs [1]) ; let ab_23 = unpackhi_epi32 (vecs [0] , vecs [1]) ; let cd_01 = unpacklo_epi32 (vecs [2] , vecs [3]) ; let cd_23 = unpackhi_epi32 (vecs [2] , vecs [3]) ; let abcd_0 = unpacklo_epi64 (ab_01 , cd_01) ; let abcd_1 = unpackhi_epi64 (ab_01 , cd_01) ; let abcd_2 = unpacklo_epi64 (ab_23 , cd_23) ; let abcd_3 = unpackhi_epi64 (ab_23 , cd_23) ; vecs [0] = abcd_0 ; vecs [1] = abcd_1 ; vecs [2] = abcd_2 ; vecs [3] = abcd_3 ; }
    };
}

transpose_vecs!();