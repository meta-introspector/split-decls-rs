// Generated macro for macro_2474 (macro)
macro_rules! Depcrate_ntrtlmacro_2474 {
() => {
// Module: crate::ntrtl
// Provides: {"macro_2474"}
// Dependencies: {}
EXTERN ! { extern "system" { fn RtlGetPersistedStateLocation (SourceID : PCWSTR , CustomValue : PCWSTR , DefaultPath : PCWSTR , StateLocationType : STATE_LOCATION_TYPE , TargetPath : PWCHAR , BufferLengthIn : ULONG , BufferLengthOut : PULONG ,) -> NTSTATUS ; fn RtlIsCloudFilesPlaceholder (FileAttributes : ULONG , ReparseTag : ULONG ,) -> BOOLEAN ; fn RtlIsPartialPlaceholder (FileAttributes : ULONG , ReparseTag : ULONG ,) -> BOOLEAN ; fn RtlIsPartialPlaceholderFileHandle (FileHandle : HANDLE , IsPartialPlaceholder : PBOOLEAN ,) -> NTSTATUS ; fn RtlIsPartialPlaceholderFileInfo (InfoBuffer : * const c_void , InfoClass : FILE_INFORMATION_CLASS , IsPartialPlaceholder : PBOOLEAN ,) -> NTSTATUS ; fn RtlIsNonEmptyDirectoryReparsePointAllowed (ReparseTag : ULONG ,) -> BOOLEAN ; fn RtlAppxIsFileOwnedByTrustedInstaller (FileHandle : HANDLE , IsFileOwnedByTrustedInstaller : PBOOLEAN ,) -> NTSTATUS ; } }
};
}
