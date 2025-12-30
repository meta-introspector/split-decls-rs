// Generated macro for macro_2775 (macro)
macro_rules! Depcrate_ntsammacro_2775 {
() => {
// Module: crate::ntsam
// Provides: {"macro_2775"}
// Dependencies: {}
EXTERN ! { extern "system" { fn SamRegisterObjectChangeNotification (ObjectType : SECURITY_DB_OBJECT_TYPE , NotificationEventHandle : HANDLE ,) -> NTSTATUS ; fn SamUnregisterObjectChangeNotification (ObjectType : SECURITY_DB_OBJECT_TYPE , NotificationEventHandle : HANDLE ,) -> NTSTATUS ; } }
};
}
