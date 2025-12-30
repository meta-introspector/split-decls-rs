// Generated macro for macro_2224 (macro)
macro_rules! Depcrate_ntrtlmacro_2224 {
() => {
// Module: crate::ntrtl
// Provides: {"macro_2224"}
// Dependencies: {}
EXTERN ! { extern "system" { fn RtlImageNtHeaderEx (Flags : ULONG , BaseOfImage : PVOID , Size : ULONG64 , OutHeaders : * mut PIMAGE_NT_HEADERS ,) -> NTSTATUS ; fn RtlAddressInSectionTable (NtHeaders : PIMAGE_NT_HEADERS , BaseOfImage : PVOID , VirtualAddress : ULONG ,) -> PVOID ; fn RtlSectionTableFromVirtualAddress (NtHeaders : PIMAGE_NT_HEADERS , BaseOfImage : PVOID , VirtualAddress : ULONG ,) -> PIMAGE_SECTION_HEADER ; fn RtlImageDirectoryEntryToData (BaseOfImage : PVOID , MappedAsImage : BOOLEAN , DirectoryEntry : USHORT , Size : PULONG ,) -> PVOID ; fn RtlImageRvaToSection (NtHeaders : PIMAGE_NT_HEADERS , BaseOfImage : PVOID , Rva : ULONG ,) -> PIMAGE_SECTION_HEADER ; fn RtlImageRvaToVa (NtHeaders : PIMAGE_NT_HEADERS , BaseOfImage : PVOID , Rva : ULONG , LastRvaSection : * mut PIMAGE_SECTION_HEADER ,) -> PVOID ; fn RtlFindExportedRoutineByName (BaseOfImage : PVOID , RoutineName : PSTR ,) -> PVOID ; fn RtlGuardCheckLongJumpTarget (PcValue : PVOID , IsFastFail : BOOL , IsLongJumpTarget : PBOOL ,) -> NTSTATUS ; fn RtlCompareMemoryUlong (Source : PVOID , Length : SIZE_T , Pattern : ULONG ,) -> SIZE_T ; fn RtlFillMemoryUlong (Destination : PVOID , Length : SIZE_T , Pattern : ULONG ,) ; fn RtlFillMemoryUlonglong (Destination : PVOID , Length : SIZE_T , Pattern : ULONGLONG ,) ; fn RtlCreateEnvironment (CloneCurrentEnvironment : BOOLEAN , Environment : * mut PVOID ,) -> NTSTATUS ; } }
};
}
