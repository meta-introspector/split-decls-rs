// Generated macro for copy_match_to_slots (function)
macro_rules! Depcrate_meta_strategycopy_match_to_slots {
() => {
// Module: crate::meta::strategy
// Provides: {"copy_match_to_slots"}
// Dependencies: {}
# [doc = " Copies the offsets in the given match to the corresponding positions in"] # [doc = " `slots`."] # [doc = ""] # [doc = " In effect, this sets the slots corresponding to the implicit group for the"] # [doc = " pattern in the given match. If the indices for the corresponding slots do"] # [doc = " not exist, then no slots are set."] # [doc = ""] # [doc = " This is useful when the caller provides slots (or captures), but you use a"] # [doc = " regex engine that doesn't operate on slots (like a lazy DFA). This function"] # [doc = " lets you map the match you get back to the slots provided by the caller."] # [cfg_attr (feature = "perf-inline" , inline (always))] fn copy_match_to_slots (m : Match , slots : & mut [Option < NonMaxUsize >]) { let slot_start = m . pattern () . as_usize () * 2 ; let slot_end = slot_start + 1 ; if let Some (slot) = slots . get_mut (slot_start) { * slot = NonMaxUsize :: new (m . start ()) ; } if let Some (slot) = slots . get_mut (slot_end) { * slot = NonMaxUsize :: new (m . end ()) ; } }
};
}
