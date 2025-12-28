macro_rules! macro_137 {
    () => {
        impl_binop ! (u32x4_sse2 , Add , add , _mm_add_epi32) ;
    };
}

macro_137!()