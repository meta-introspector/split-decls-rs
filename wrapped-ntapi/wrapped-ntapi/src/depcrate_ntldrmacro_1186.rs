// Generated macro for macro_1186 (macro)
macro_rules! Depcrate_ntldrmacro_1186 {
() => {
// Module: crate::ntldr
// Provides: {"macro_1186"}
// Dependencies: {}
EXTERN ! { extern "system" { fn LdrEnumerateLoadedModules (ReservedFlag : BOOLEAN , EnumProc : PLDR_ENUM_CALLBACK , Context : PVOID ,) -> NTSTATUS ; fn LdrOpenImageFileOptionsKey (SubKey : PUNICODE_STRING , Wow64 : BOOLEAN , NewKeyHandle : PHANDLE ,) -> NTSTATUS ; fn LdrQueryImageFileKeyOption (KeyHandle : HANDLE , ValueName : PCWSTR , Type : ULONG , Buffer : PVOID , BufferSize : ULONG , ReturnedLength : PULONG ,) -> NTSTATUS ; fn LdrQueryImageFileExecutionOptions (SubKey : PUNICODE_STRING , ValueName : PCWSTR , ValueSize : ULONG , Buffer : PVOID , BufferSize : ULONG , ReturnedLength : PULONG ,) -> NTSTATUS ; fn LdrQueryImageFileExecutionOptionsEx (SubKey : PUNICODE_STRING , ValueName : PCWSTR , Type : ULONG , Buffer : PVOID , BufferSize : ULONG , ReturnedLength : PULONG , Wow64 : BOOLEAN ,) -> NTSTATUS ; } }
};
}
