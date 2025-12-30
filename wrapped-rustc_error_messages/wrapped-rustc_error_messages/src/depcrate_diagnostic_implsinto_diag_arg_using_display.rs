// Generated macro for into_diag_arg_using_display (macro)
macro_rules! Depcrate_diagnostic_implsinto_diag_arg_using_display {
() => {
// Module: crate::diagnostic_impls
// Provides: {"into_diag_arg_using_display"}
// Dependencies: {}
# [macro_export] macro_rules ! into_diag_arg_using_display { ($ ($ ty : ty) ,+ $ (,) ?) => { $ (impl $ crate :: IntoDiagArg for $ ty { fn into_diag_arg (self , path : & mut Option < std :: path :: PathBuf >) -> $ crate :: DiagArgValue { self . to_string () . into_diag_arg (path) } }) + } }
};
}
