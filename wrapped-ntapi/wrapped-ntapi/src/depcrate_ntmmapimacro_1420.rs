// Generated macro for macro_1420 (macro)
macro_rules! Depcrate_ntmmapimacro_1420 {
() => {
// Module: crate::ntmmapi
// Provides: {"macro_1420"}
// Dependencies: {}
EXTERN ! { extern "system" { fn NtAllocateVirtualMemory (ProcessHandle : HANDLE , BaseAddress : * mut PVOID , ZeroBits : ULONG_PTR , RegionSize : PSIZE_T , AllocationType : ULONG , Protect : ULONG ,) -> NTSTATUS ; fn NtFreeVirtualMemory (ProcessHandle : HANDLE , BaseAddress : * mut PVOID , RegionSize : PSIZE_T , FreeType : ULONG ,) -> NTSTATUS ; fn NtReadVirtualMemory (ProcessHandle : HANDLE , BaseAddress : PVOID , Buffer : PVOID , BufferSize : SIZE_T , NumberOfBytesRead : PSIZE_T ,) -> NTSTATUS ; fn NtWriteVirtualMemory (ProcessHandle : HANDLE , BaseAddress : PVOID , Buffer : PVOID , BufferSize : SIZE_T , NumberOfBytesWritten : PSIZE_T ,) -> NTSTATUS ; fn NtProtectVirtualMemory (ProcessHandle : HANDLE , BaseAddress : * mut PVOID , RegionSize : PSIZE_T , NewProtect : ULONG , OldProtect : PULONG ,) -> NTSTATUS ; fn NtQueryVirtualMemory (ProcessHandle : HANDLE , BaseAddress : PVOID , MemoryInformationClass : MEMORY_INFORMATION_CLASS , MemoryInformation : PVOID , MemoryInformationLength : SIZE_T , ReturnLength : PSIZE_T ,) -> NTSTATUS ; } }
};
}
