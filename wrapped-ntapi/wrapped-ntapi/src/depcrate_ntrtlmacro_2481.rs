// Generated macro for macro_2481 (macro)
macro_rules! Depcrate_ntrtlmacro_2481 {
() => {
// Module: crate::ntrtl
// Provides: {"macro_2481"}
// Dependencies: {}
EXTERN ! { extern "system" { fn RtlCreateBootStatusDataFile () -> NTSTATUS ; fn RtlLockBootStatusData (FileHandle : PHANDLE ,) -> NTSTATUS ; fn RtlUnlockBootStatusData (FileHandle : HANDLE ,) -> NTSTATUS ; fn RtlGetSetBootStatusData (FileHandle : HANDLE , Read : BOOLEAN , DataClass : RTL_BSD_ITEM_TYPE , Buffer : PVOID , BufferSize : ULONG , ReturnLength : PULONG ,) -> NTSTATUS ; fn RtlCheckBootStatusIntegrity (FileHandle : HANDLE , Verified : PBOOLEAN ,) -> NTSTATUS ; fn RtlCheckPortableOperatingSystem (IsPortable : PBOOLEAN ,) -> NTSTATUS ; fn RtlSetPortableOperatingSystem (IsPortable : BOOLEAN ,) -> NTSTATUS ; } }
};
}
