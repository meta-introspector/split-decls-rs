macro_rules! deps {
    () => {
        Block!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl PartialEq for Block { # [inline] fn eq (& self , other : & Self) -> bool { unsafe { # [cfg (not (target_feature = "sse4.1"))] { _mm_movemask_epi8 (_mm_cmpeq_epi8 (self . 0 , other . 0)) == 0xffff } # [cfg (target_feature = "sse4.1")] { let neq = _mm_xor_si128 (self . 0 , other . 0) ; _mm_test_all_zeros (neq , neq) == 1 } } } }
    };
}

impl_20!()