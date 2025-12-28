macro_rules! deps {
    () => {
        StateID!();
    };
}

macro_rules! u32s_to_state_ids {
    () => {
        deps!();
        # [doc = " Safely converts a `&[u32]` to `&[StateID]` with zero cost."] # [cfg_attr (feature = "perf-inline" , inline (always))] pub (crate) fn u32s_to_state_ids (slice : & [u32]) -> & [StateID] { unsafe { core :: slice :: from_raw_parts (slice . as_ptr () . cast :: < StateID > () , slice . len () ,) } }
    };
}

u32s_to_state_ids!();