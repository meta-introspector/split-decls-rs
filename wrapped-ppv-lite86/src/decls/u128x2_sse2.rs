macro_rules! deps {
    () => {
        G0!();
    };
}

macro_rules! u128x2_sse2 {
    () => {
        deps!();
        # [allow (non_camel_case_types)] pub type u128x2_sse2 < S3 , S4 , NI > = x2 < u128x1_sse2 < S3 , S4 , NI > , G0 > ;
    };
}

u128x2_sse2!()