// Generated macro for macro_2228 (macro)
macro_rules! Depcrate_ntrtlmacro_2228 {
() => {
// Module: crate::ntrtl
// Provides: {"macro_2228"}
// Dependencies: {}
EXTERN ! { extern "system" { fn RtlCreateEnvironmentEx (SourceEnv : PVOID , Environment : * mut PVOID , Flags : ULONG ,) -> NTSTATUS ; fn RtlDestroyEnvironment (Environment : PVOID ,) -> NTSTATUS ; fn RtlSetCurrentEnvironment (Environment : PVOID , PreviousEnvironment : * mut PVOID ,) -> NTSTATUS ; fn RtlSetEnvironmentVar (Environment : * mut PWSTR , Name : PWSTR , NameLength : SIZE_T , Value : PWSTR , ValueLength : SIZE_T ,) -> NTSTATUS ; fn RtlSetEnvironmentVariable (Environment : * mut PVOID , Name : PUNICODE_STRING , Value : PUNICODE_STRING ,) -> NTSTATUS ; fn RtlQueryEnvironmentVariable (Environment : PVOID , Name : PWSTR , NameLength : SIZE_T , Value : PWSTR , ValueLength : SIZE_T , ReturnLength : PSIZE_T ,) -> NTSTATUS ; fn RtlQueryEnvironmentVariable_U (Environment : PVOID , Name : PUNICODE_STRING , Value : PUNICODE_STRING ,) -> NTSTATUS ; fn RtlExpandEnvironmentStrings (Environment : PVOID , Src : PWSTR , SrcLength : SIZE_T , Dst : PWSTR , DstLength : SIZE_T , ReturnLength : PSIZE_T ,) -> NTSTATUS ; fn RtlExpandEnvironmentStrings_U (Environment : PVOID , Source : PUNICODE_STRING , Destination : PUNICODE_STRING , ReturnedLength : PULONG ,) -> NTSTATUS ; fn RtlSetEnvironmentStrings (NewEnvironment : PWCHAR , NewEnvironmentSize : SIZE_T ,) -> NTSTATUS ; } }
};
}
