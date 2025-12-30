// Generated macro for NtWaitForSingleObject (function)
macro_rules! Depcrate_bool_eventNtWaitForSingleObject {
() => {
// Module: crate::bool_event
// Provides: {"NtWaitForSingleObject"}
// Dependencies: {}
# [inline] pub unsafe fn NtWaitForSingleObject (handle : windows :: Win32 :: Foundation :: HANDLE , alertable : bool , timeout : * mut i64 ,) -> windows_core :: NTSTATUS { windows_core :: link ! ("ntdll.dll" "system" fn NtWaitForSingleObject (handle : windows :: Win32 :: Foundation :: HANDLE , alertable : bool , timeout : * mut i64) -> windows_core :: NTSTATUS) ; unsafe { NtWaitForSingleObject (handle , alertable , timeout as _) } }
};
}
