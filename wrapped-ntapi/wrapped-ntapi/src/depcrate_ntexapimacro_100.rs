// Generated macro for macro_100 (macro)
macro_rules! Depcrate_ntexapimacro_100 {
() => {
// Module: crate::ntexapi
// Provides: {"macro_100"}
// Dependencies: {}
EXTERN ! { extern "system" { fn NtDelayExecution (Alertable : BOOLEAN , DelayInterval : PLARGE_INTEGER ,) -> NTSTATUS ; fn NtQuerySystemEnvironmentValue (VariableName : PUNICODE_STRING , VariableValue : PWSTR , ValueLength : USHORT , ReturnLength : PUSHORT ,) -> NTSTATUS ; fn NtSetSystemEnvironmentValue (VariableName : PUNICODE_STRING , VariableValue : PUNICODE_STRING ,) -> NTSTATUS ; fn NtQuerySystemEnvironmentValueEx (VariableName : PUNICODE_STRING , VendorGuid : LPGUID , Value : PVOID , ValueLength : PULONG , Attributes : PULONG ,) -> NTSTATUS ; fn NtSetSystemEnvironmentValueEx (VariableName : PUNICODE_STRING , VendorGuid : LPGUID , Value : PVOID , ValueLength : ULONG , Attributes : ULONG ,) -> NTSTATUS ; fn NtEnumerateSystemEnvironmentValuesEx (InformationClass : ULONG , Buffer : PVOID , BufferLength : PULONG ,) -> NTSTATUS ; } }
};
}
