// Generated macro for macro_2217 (macro)
macro_rules! Depcrate_ntrtlmacro_2217 {
() => {
// Module: crate::ntrtl
// Provides: {"macro_2217"}
// Dependencies: {}
# [cfg (any (target_arch = "x86_64" , target_arch = "aarch64"))] EXTERN ! { extern "system" { fn RtlWow64GetThreadContext (ThreadHandle : HANDLE , ThreadContext : PWOW64_CONTEXT ,) -> NTSTATUS ; fn RtlWow64SetThreadContext (ThreadHandle : HANDLE , ThreadContext : PWOW64_CONTEXT ,) -> NTSTATUS ; } }
};
}
