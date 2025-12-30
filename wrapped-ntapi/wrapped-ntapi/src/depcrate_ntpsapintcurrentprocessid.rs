// Generated macro for NtCurrentProcessId (function)
macro_rules! Depcrate_ntpsapiNtCurrentProcessId {
() => {
// Module: crate::ntpsapi
// Provides: {"NtCurrentProcessId"}
// Dependencies: {}
# [inline] # [cfg (not (target_arch = "aarch64"))] pub unsafe fn NtCurrentProcessId () -> HANDLE { (* NtCurrentTeb ()) . ClientId . UniqueProcess }
};
}
