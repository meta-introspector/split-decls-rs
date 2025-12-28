macro_rules! deps {
    () => {
        G0!();
    };
}

macro_rules! u32x4x2_sse2 {
    () => {
        deps!();
        # [allow (non_camel_case_types)] pub type u32x4x2_sse2 < S3 , S4 , NI > = x2 < u32x4_sse2 < S3 , S4 , NI > , G0 > ;
    };
}

u32x4x2_sse2!()