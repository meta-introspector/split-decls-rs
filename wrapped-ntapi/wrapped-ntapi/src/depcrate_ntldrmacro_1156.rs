// Generated macro for macro_1156 (macro)
macro_rules! Depcrate_ntldrmacro_1156 {
() => {
// Module: crate::ntldr
// Provides: {"macro_1156"}
// Dependencies: {}
EXTERN ! { extern "system" { fn LdrRegisterDllNotification (Flags : ULONG , NotificationFunction : PLDR_DLL_NOTIFICATION_FUNCTION , Context : PVOID , Cookie : * mut PVOID ,) -> NTSTATUS ; fn LdrUnregisterDllNotification (Cookie : PVOID ,) -> NTSTATUS ; } }
};
}
