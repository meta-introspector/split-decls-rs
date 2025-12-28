macro_rules! u128x4_sse2 {
    () => {
        # [allow (non_camel_case_types)] pub type u128x4_sse2 < S3 , S4 , NI > = x4 < u128x1_sse2 < S3 , S4 , NI > > ;
    };
}

u128x4_sse2!();