macro_rules! macro_138 {
    () => {
        impl_binop ! (u64x2_sse2 , Add , add , _mm_add_epi64) ;
    };
}

macro_138!()