// Generated macro for u32s_to_state_ids_mut (function)
macro_rules! Depcrate_util_wireu32s_to_state_ids_mut {
() => {
// Module: crate::util::wire
// Provides: {"u32s_to_state_ids_mut"}
// Dependencies: {}
# [doc = " Safely converts a `&mut [u32]` to `&mut [StateID]` with zero cost."] pub (crate) fn u32s_to_state_ids_mut (slice : & mut [u32]) -> & mut [StateID] { unsafe { core :: slice :: from_raw_parts_mut (slice . as_mut_ptr () . cast :: < StateID > () , slice . len () ,) } }
};
}
