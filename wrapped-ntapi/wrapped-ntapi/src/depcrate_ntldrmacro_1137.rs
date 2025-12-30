// Generated macro for macro_1137 (macro)
macro_rules! Depcrate_ntldrmacro_1137 {
() => {
// Module: crate::ntldr
// Provides: {"macro_1137"}
// Dependencies: {}
EXTERN ! { extern "system" { fn LdrUnlockLoaderLock (Flags : ULONG , Cookie : PVOID ,) -> NTSTATUS ; fn LdrRelocateImage (NewBase : PVOID , LoaderName : PSTR , Success : NTSTATUS , Conflict : NTSTATUS , Invalid : NTSTATUS ,) -> NTSTATUS ; fn LdrRelocateImageWithBias (NewBase : PVOID , Bias : LONGLONG , LoaderName : PSTR , Success : NTSTATUS , Conflict : NTSTATUS , Invalid : NTSTATUS ,) -> NTSTATUS ; fn LdrProcessRelocationBlock (VA : ULONG_PTR , SizeOfBlock : ULONG , NextOffset : PUSHORT , Diff : LONG_PTR ,) -> PIMAGE_BASE_RELOCATION ; fn LdrVerifyMappedImageMatchesChecksum (BaseAddress : PVOID , NumberOfBytes : SIZE_T , FileLength : ULONG ,) -> BOOLEAN ; } }
};
}
