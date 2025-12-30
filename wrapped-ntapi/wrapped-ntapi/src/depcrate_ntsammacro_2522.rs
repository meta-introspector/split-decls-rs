// Generated macro for macro_2522 (macro)
macro_rules! Depcrate_ntsammacro_2522 {
() => {
// Module: crate::ntsam
// Provides: {"macro_2522"}
// Dependencies: {}
EXTERN ! { extern "system" { fn SamConnect (ServerName : PUNICODE_STRING , ServerHandle : PSAM_HANDLE , DesiredAccess : ACCESS_MASK , ObjectAttributes : POBJECT_ATTRIBUTES ,) -> NTSTATUS ; fn SamShutdownSamServer (ServerHandle : SAM_HANDLE ,) -> NTSTATUS ; } }
};
}
