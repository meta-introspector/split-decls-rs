macro_rules! deps {
    () => {
        LaneWords4!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl < W : Copy + LaneWords4 , G : Copy > LaneWords4 for x2 < W , G > { # [inline (always)] fn shuffle_lane_words2301 (self) -> Self { Self :: new ([self . 0 [0] . shuffle_lane_words2301 () , self . 0 [1] . shuffle_lane_words2301 () ,]) } # [inline (always)] fn shuffle_lane_words1230 (self) -> Self { Self :: new ([self . 0 [0] . shuffle_lane_words1230 () , self . 0 [1] . shuffle_lane_words1230 () ,]) } # [inline (always)] fn shuffle_lane_words3012 (self) -> Self { Self :: new ([self . 0 [0] . shuffle_lane_words3012 () , self . 0 [1] . shuffle_lane_words3012 () ,]) } }
    };
}

impl_31!()