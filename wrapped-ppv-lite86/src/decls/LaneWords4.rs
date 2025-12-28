macro_rules! LaneWords4 {
    () => {
        # [doc = " A vector composed one or more lanes each composed of four words."] pub trait LaneWords4 { fn shuffle_lane_words1230 (self) -> Self ; fn shuffle_lane_words2301 (self) -> Self ; fn shuffle_lane_words3012 (self) -> Self ; }
    };
}

LaneWords4!();