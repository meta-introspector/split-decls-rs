// Generated macro for macro_2193 (macro)
macro_rules! Depcrate_ntrtlmacro_2193 {
() => {
// Module: crate::ntrtl
// Provides: {"macro_2193"}
// Dependencies: {}
EXTERN ! { extern "system" { fn RtlCreateUserProcess (NtImagePathName : PUNICODE_STRING , AttributesDeprecated : ULONG , ProcessParameters : PRTL_USER_PROCESS_PARAMETERS , ProcessSecurityDescriptor : PSECURITY_DESCRIPTOR , ThreadSecurityDescriptor : PSECURITY_DESCRIPTOR , ParentProcess : HANDLE , InheritHandles : BOOLEAN , DebugPort : HANDLE , TokenHandle : HANDLE , ProcessInformation : PRTL_USER_PROCESS_INFORMATION ,) -> NTSTATUS ; fn RtlCreateUserProcessEx (NtImagePathName : PUNICODE_STRING , ProcessParameters : PRTL_USER_PROCESS_PARAMETERS , InheritHandles : BOOLEAN , Flags : ULONG , ProcessInformation : PRTL_USER_PROCESS_INFORMATION ,) -> NTSTATUS ; fn RtlExitUserProcess (ExitStatus : NTSTATUS ,) ; } }
};
}
