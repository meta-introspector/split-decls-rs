// Generated macro for macro_2202 (macro)
macro_rules! Depcrate_ntrtlmacro_2202 {
() => {
// Module: crate::ntrtl
// Provides: {"macro_2202"}
// Dependencies: {}
EXTERN ! { extern "system" { fn RtlValidProcessProtection (ProcessProtection : PS_PROTECTION ,) -> BOOLEAN ; fn RtlTestProtectedAccess (Source : PS_PROTECTION , Target : PS_PROTECTION ,) -> BOOLEAN ; fn RtlIsCurrentProcess (ProcessHandle : HANDLE ,) -> BOOLEAN ; fn RtlIsCurrentThread (ThreadHandle : HANDLE ,) -> BOOLEAN ; } }
};
}
