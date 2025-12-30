// Generated macro for NtCurrentPeb (function)
macro_rules! Depcrate_ntpsapiNtCurrentPeb {
() => {
// Module: crate::ntpsapi
// Provides: {"NtCurrentPeb"}
// Dependencies: {}
# [inline] # [cfg (not (target_arch = "aarch64"))] pub unsafe fn NtCurrentPeb () -> PPEB { (* NtCurrentTeb ()) . ProcessEnvironmentBlock }
};
}
