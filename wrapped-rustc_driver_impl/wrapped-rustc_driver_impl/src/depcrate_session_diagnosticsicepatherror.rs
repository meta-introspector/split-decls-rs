// Generated macro for IcePathError (struct)
macro_rules! Depcrate_session_diagnosticsIcePathError {
() => {
// Module: crate::session_diagnostics
// Provides: {"IcePathError"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (driver_impl_ice_path_error)] pub (crate) struct IcePathError { pub path : std :: path :: PathBuf , pub error : String , # [subdiagnostic] pub env_var : Option < IcePathErrorEnv > , }
};
}
