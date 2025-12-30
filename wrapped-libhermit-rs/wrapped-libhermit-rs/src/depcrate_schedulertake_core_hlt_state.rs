// Generated macro for take_core_hlt_state (function)
macro_rules! Depcrate_schedulertake_core_hlt_state {
() => {
// Module: crate::scheduler
// Provides: {"take_core_hlt_state"}
// Dependencies: {}
# [inline] # [cfg (all (target_arch = "x86_64" , feature = "smp" , not (feature = "idle-poll")))] pub (crate) fn take_core_hlt_state (core_id : CoreId) -> bool { CORE_HLT_STATE . lock () [usize :: try_from (core_id) . unwrap ()] . swap (false , Ordering :: Acquire) }
};
}
