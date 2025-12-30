// Generated macro for into_diag_arg_for_number (macro)
macro_rules! Depcrate_diagnostic_implsinto_diag_arg_for_number {
() => {
// Module: crate::diagnostic_impls
// Provides: {"into_diag_arg_for_number"}
// Dependencies: {}
macro_rules ! into_diag_arg_for_number { ($ ($ ty : ty) ,+ $ (,) ?) => { $ (impl $ crate :: IntoDiagArg for $ ty { fn into_diag_arg (self , path : & mut Option < std :: path :: PathBuf >) -> $ crate :: DiagArgValue { # [allow (irrefutable_let_patterns)] if let Ok (n) = TryInto ::< i32 >:: try_into (self) { $ crate :: DiagArgValue :: Number (n) } else { self . to_string () . into_diag_arg (path) } } }) + } }
};
}
