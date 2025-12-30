// Generated macro for macro_2805 (macro)
macro_rules! Depcrate_ntsammacro_2805 {
() => {
// Module: crate::ntsam
// Provides: {"macro_2805"}
// Dependencies: {}
EXTERN ! { extern "system" { fn SamValidatePassword (ServerName : PUNICODE_STRING , ValidationType : PASSWORD_POLICY_VALIDATION_TYPE , InputArg : PSAM_VALIDATE_INPUT_ARG , OutputArg : * mut PSAM_VALIDATE_OUTPUT_ARG ,) -> NTSTATUS ; } }
};
}
