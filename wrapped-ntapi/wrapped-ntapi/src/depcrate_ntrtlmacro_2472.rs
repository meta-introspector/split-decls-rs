// Generated macro for macro_2472 (macro)
macro_rules! Depcrate_ntrtlmacro_2472 {
() => {
// Module: crate::ntrtl
// Provides: {"macro_2472"}
// Dependencies: {}
EXTERN ! { extern "system" { fn RtlGetAppContainerSidType (AppContainerSid : PSID , AppContainerSidType : PAPPCONTAINER_SID_TYPE ,) -> NTSTATUS ; fn RtlFlsAlloc (Callback : PFLS_CALLBACK_FUNCTION , FlsIndex : PULONG ,) -> NTSTATUS ; fn RtlFlsFree (FlsIndex : ULONG ,) -> NTSTATUS ; } }
};
}
