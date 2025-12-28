macro_rules! deps {
    () => {
        G1!();
    };
}

macro_rules! u64x4_sse2 {
    () => {
        deps!();
        # [allow (non_camel_case_types)] pub type u64x4_sse2 < S3 , S4 , NI > = x2 < u64x2_sse2 < S3 , S4 , NI > , G1 > ;
    };
}

u64x4_sse2!()