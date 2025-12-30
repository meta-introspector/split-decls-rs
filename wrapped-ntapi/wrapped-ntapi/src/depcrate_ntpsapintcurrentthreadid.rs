// Generated macro for NtCurrentThreadId (function)
macro_rules! Depcrate_ntpsapiNtCurrentThreadId {
() => {
// Module: crate::ntpsapi
// Provides: {"NtCurrentThreadId"}
// Dependencies: {}
# [inline] # [cfg (not (target_arch = "aarch64"))] pub unsafe fn NtCurrentThreadId () -> HANDLE { (* NtCurrentTeb ()) . ClientId . UniqueThread }
};
}
