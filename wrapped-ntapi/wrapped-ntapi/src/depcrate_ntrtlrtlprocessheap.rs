// Generated macro for RtlProcessHeap (function)
macro_rules! Depcrate_ntrtlRtlProcessHeap {
() => {
// Module: crate::ntrtl
// Provides: {"RtlProcessHeap"}
// Dependencies: {}
# [inline] # [cfg (not (target_arch = "aarch64"))] pub unsafe fn RtlProcessHeap () -> PVOID { use crate :: ntpsapi :: NtCurrentPeb ; (* NtCurrentPeb ()) . ProcessHeap }
};
}
