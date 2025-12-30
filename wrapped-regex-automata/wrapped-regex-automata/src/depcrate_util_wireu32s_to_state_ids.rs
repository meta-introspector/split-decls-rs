// Generated macro for u32s_to_state_ids (function)
macro_rules! Depcrate_util_wireu32s_to_state_ids {
() => {
// Module: crate::util::wire
// Provides: {"u32s_to_state_ids"}
// Dependencies: {}
# [doc = " Safely converts a `&[u32]` to `&[StateID]` with zero cost."] # [cfg_attr (feature = "perf-inline" , inline (always))] pub (crate) fn u32s_to_state_ids (slice : & [u32]) -> & [StateID] { unsafe { core :: slice :: from_raw_parts (slice . as_ptr () . cast :: < StateID > () , slice . len () ,) } }
};
}
