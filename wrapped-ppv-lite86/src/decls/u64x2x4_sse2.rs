macro_rules! u64x2x4_sse2 {
    () => {
        # [allow (non_camel_case_types)] pub type u64x2x4_sse2 < S3 , S4 , NI > = x4 < u64x2_sse2 < S3 , S4 , NI > > ;
    };
}

u64x2x4_sse2!()