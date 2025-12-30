// Generated macro for ErrorCallingDllTool (struct)
macro_rules! Depcrate_errorsErrorCallingDllTool {
() => {
// Module: crate::errors
// Provides: {"ErrorCallingDllTool"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (codegen_ssa_error_calling_dlltool)] pub (crate) struct ErrorCallingDllTool < 'a > { pub dlltool_path : Cow < 'a , str > , pub error : std :: io :: Error , }
};
}
