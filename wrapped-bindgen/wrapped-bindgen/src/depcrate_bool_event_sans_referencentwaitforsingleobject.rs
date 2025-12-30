// Generated macro for NtWaitForSingleObject (function)
macro_rules! Depcrate_bool_event_sans_referenceNtWaitForSingleObject {
() => {
// Module: crate::bool_event_sans_reference
// Provides: {"NtWaitForSingleObject"}
// Dependencies: {}
# [inline] pub unsafe fn NtWaitForSingleObject (handle : HANDLE , alertable : bool , timeout : * mut i64 ,) -> windows_core :: NTSTATUS { windows_core :: link ! ("ntdll.dll" "system" fn NtWaitForSingleObject (handle : HANDLE , alertable : bool , timeout : * mut i64) -> windows_core :: NTSTATUS) ; unsafe { NtWaitForSingleObject (handle , alertable , timeout as _) } }
};
}
