// Generated macro for macro_70 (macro)
macro_rules! Depcrate_os_iocp_afdmacro_70 {
() => {
// Module: crate::os::iocp::afd
// Provides: {"macro_70"}
// Dependencies: {}
define_ntdll_import ! { # [doc = " Cancels an ongoing I/O operation."] fn NtCancelIoFileEx (FileHandle : HANDLE , IoRequestToCancel : * mut IO_STATUS_BLOCK , IoStatusBlock : * mut IO_STATUS_BLOCK) -> NTSTATUS ; # [doc = " Opens or creates a file handle."] # [allow (clippy :: too_many_arguments)] fn NtCreateFile (FileHandle : * mut HANDLE , DesiredAccess : u32 , ObjectAttributes : * mut OBJECT_ATTRIBUTES , IoStatusBlock : * mut IO_STATUS_BLOCK , AllocationSize : * mut i64 , FileAttributes : u32 , ShareAccess : u32 , CreateDisposition : u32 , CreateOptions : u32 , EaBuffer : * mut () , EaLength : u32) -> NTSTATUS ; # [doc = " Runs an I/O control on a file handle."] # [doc = ""] # [doc = " Practically equivalent to `ioctl`."] # [allow (clippy :: too_many_arguments)] fn NtDeviceIoControlFile (FileHandle : HANDLE , Event : HANDLE , ApcRoutine : * mut () , ApcContext : * mut () , IoStatusBlock : * mut IO_STATUS_BLOCK , IoControlCode : u32 , InputBuffer : * mut () , InputBufferLength : u32 , OutputBuffer : * mut () , OutputBufferLength : u32) -> NTSTATUS ; # [doc = " Converts `NTSTATUS` to a DOS error code."] fn RtlNtStatusToDosError (Status : NTSTATUS) -> u32 ; }
};
}
