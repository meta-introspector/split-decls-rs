macro_rules! deps {
    () => {
        LaneWords4!();
    };
}

macro_rules! impl_150 {
    () => {
        deps!();
        impl < S3 , S4 , NI > LaneWords4 for u32x4_sse2 < S3 , S4 , NI > { # [inline (always)] fn shuffle_lane_words2301 (self) -> Self { self . shuffle2301 () } # [inline (always)] fn shuffle_lane_words1230 (self) -> Self { self . shuffle1230 () } # [inline (always)] fn shuffle_lane_words3012 (self) -> Self { self . shuffle3012 () } }
    };
}

impl_150!()