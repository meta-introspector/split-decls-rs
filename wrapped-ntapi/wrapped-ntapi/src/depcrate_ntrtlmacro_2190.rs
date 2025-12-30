// Generated macro for macro_2190 (macro)
macro_rules! Depcrate_ntrtlmacro_2190 {
() => {
// Module: crate::ntrtl
// Provides: {"macro_2190"}
// Dependencies: {}
EXTERN ! { extern "system" { fn RtlCreateProcessParameters (pProcessParameters : * mut PRTL_USER_PROCESS_PARAMETERS , ImagePathName : PUNICODE_STRING , DllPath : PUNICODE_STRING , CurrentDirectory : PUNICODE_STRING , CommandLine : PUNICODE_STRING , Environment : PVOID , WindowTitle : PUNICODE_STRING , DesktopInfo : PUNICODE_STRING , ShellInfo : PUNICODE_STRING , RuntimeData : PUNICODE_STRING ,) -> NTSTATUS ; fn RtlCreateProcessParametersEx (pProcessParameters : * mut PRTL_USER_PROCESS_PARAMETERS , ImagePathName : PUNICODE_STRING , DllPath : PUNICODE_STRING , CurrentDirectory : PUNICODE_STRING , CommandLine : PUNICODE_STRING , Environment : PVOID , WindowTitle : PUNICODE_STRING , DesktopInfo : PUNICODE_STRING , ShellInfo : PUNICODE_STRING , RuntimeData : PUNICODE_STRING , Flags : ULONG ,) -> NTSTATUS ; fn RtlDestroyProcessParameters (ProcessParameters : PRTL_USER_PROCESS_PARAMETERS ,) -> NTSTATUS ; fn RtlNormalizeProcessParams (ProcessParameters : PRTL_USER_PROCESS_PARAMETERS ,) -> PRTL_USER_PROCESS_PARAMETERS ; fn RtlDeNormalizeProcessParams (ProcessParameters : PRTL_USER_PROCESS_PARAMETERS ,) -> PRTL_USER_PROCESS_PARAMETERS ; } }
};
}
