// Generated macro for macro_2511 (macro)
macro_rules! Depcrate_ntsammacro_2511 {
() => {
// Module: crate::ntsam
// Provides: {"macro_2511"}
// Dependencies: {}
EXTERN ! { extern "system" { fn SamFreeMemory (Buffer : PVOID ,) -> NTSTATUS ; fn SamCloseHandle (SamHandle : SAM_HANDLE ,) -> NTSTATUS ; fn SamSetSecurityObject (ObjectHandle : SAM_HANDLE , SecurityInformation : SECURITY_INFORMATION , SecurityDescriptor : PSECURITY_DESCRIPTOR ,) -> NTSTATUS ; fn SamQuerySecurityObject (ObjectHandle : SAM_HANDLE , SecurityInformation : SECURITY_INFORMATION , SecurityDescriptor : * mut PSECURITY_DESCRIPTOR ,) -> NTSTATUS ; fn SamRidToSid (ObjectHandle : SAM_HANDLE , Rid : ULONG , Sid : * mut PSID ,) -> NTSTATUS ; } }
};
}
