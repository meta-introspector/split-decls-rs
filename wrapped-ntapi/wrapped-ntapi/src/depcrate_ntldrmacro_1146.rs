// Generated macro for macro_1146 (macro)
macro_rules! Depcrate_ntldrmacro_1146 {
() => {
// Module: crate::ntldr
// Provides: {"macro_1146"}
// Dependencies: {}
EXTERN ! { extern "system" { fn LdrVerifyImageMatchesChecksumEx (ImageFileHandle : HANDLE , VerifyInfo : PLDR_VERIFY_IMAGE_INFO ,) -> NTSTATUS ; fn LdrQueryModuleServiceTags (DllHandle : PVOID , ServiceTagBuffer : PULONG , BufferSize : PULONG ,) -> NTSTATUS ; } }
};
}
