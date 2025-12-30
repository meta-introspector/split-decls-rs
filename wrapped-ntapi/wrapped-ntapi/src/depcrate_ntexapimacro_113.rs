// Generated macro for macro_113 (macro)
macro_rules! Depcrate_ntexapimacro_113 {
() => {
// Module: crate::ntexapi
// Provides: {"macro_113"}
// Dependencies: {}
EXTERN ! { extern "system" { fn NtAddBootEntry (BootEntry : PBOOT_ENTRY , Id : PULONG ,) -> NTSTATUS ; fn NtDeleteBootEntry (Id : ULONG ,) -> NTSTATUS ; fn NtModifyBootEntry (BootEntry : PBOOT_ENTRY ,) -> NTSTATUS ; fn NtEnumerateBootEntries (Buffer : PVOID , BufferLength : PULONG ,) -> NTSTATUS ; fn NtQueryBootEntryOrder (Ids : PULONG , Count : PULONG ,) -> NTSTATUS ; fn NtSetBootEntryOrder (Ids : PULONG , Count : ULONG ,) -> NTSTATUS ; fn NtQueryBootOptions (BootOptions : PBOOT_OPTIONS , BootOptionsLength : PULONG ,) -> NTSTATUS ; fn NtSetBootOptions (BootOptions : PBOOT_OPTIONS , FieldsToChange : ULONG ,) -> NTSTATUS ; fn NtTranslateFilePath (InputFilePath : PFILE_PATH , OutputType : ULONG , OutputFilePath : PFILE_PATH , OutputFilePathLength : PULONG ,) -> NTSTATUS ; fn NtAddDriverEntry (DriverEntry : PEFI_DRIVER_ENTRY , Id : PULONG ,) -> NTSTATUS ; fn NtDeleteDriverEntry (Id : ULONG ,) -> NTSTATUS ; fn NtModifyDriverEntry (DriverEntry : PEFI_DRIVER_ENTRY ,) -> NTSTATUS ; fn NtEnumerateDriverEntries (Buffer : PVOID , BufferLength : PULONG ,) -> NTSTATUS ; fn NtQueryDriverEntryOrder (Ids : PULONG , Count : PULONG ,) -> NTSTATUS ; fn NtSetDriverEntryOrder (Ids : PULONG , Count : ULONG ,) -> NTSTATUS ; } }
};
}
