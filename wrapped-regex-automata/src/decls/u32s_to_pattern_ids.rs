macro_rules! deps {
    () => {
        PatternID!();
    };
}

macro_rules! u32s_to_pattern_ids {
    () => {
        deps!();
        # [doc = " Safely converts a `&[u32]` to `&[PatternID]` with zero cost."] # [cfg_attr (feature = "perf-inline" , inline (always))] pub (crate) fn u32s_to_pattern_ids (slice : & [u32]) -> & [PatternID] { unsafe { core :: slice :: from_raw_parts (slice . as_ptr () . cast :: < PatternID > () , slice . len () ,) } }
    };
}

u32s_to_pattern_ids!()