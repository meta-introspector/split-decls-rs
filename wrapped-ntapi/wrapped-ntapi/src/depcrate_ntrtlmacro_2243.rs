// Generated macro for macro_2243 (macro)
macro_rules! Depcrate_ntrtlmacro_2243 {
() => {
// Module: crate::ntrtl
// Provides: {"macro_2243"}
// Dependencies: {}
EXTERN ! { extern "system" { fn RtlGenerate8dot3Name (Name : PCUNICODE_STRING , AllowExtendedCharacters : BOOLEAN , Context : PGENERATE_NAME_CONTEXT , Name8dot3 : PUNICODE_STRING ,) -> NTSTATUS ; fn RtlComputePrivatizedDllName_U (DllName : PUNICODE_STRING , RealName : PUNICODE_STRING , LocalName : PUNICODE_STRING ,) -> NTSTATUS ; fn RtlGetSearchPath (SearchPathA : * mut PWSTR ,) -> BOOLEAN ; fn RtlSetSearchPathMode (Flags : ULONG ,) -> NTSTATUS ; fn RtlGetExePath () -> PWSTR ; fn RtlGetNtSystemRoot () -> PWSTR ; fn RtlAreLongPathsEnabled () -> BOOLEAN ; fn RtlIsThreadWithinLoaderCallout () -> BOOLEAN ; fn RtlDllShutdownInProgress () -> BOOLEAN ; } }
};
}
