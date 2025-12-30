// Generated macro for warn_or_err (macro)
macro_rules! Depcratewarn_or_err {
() => {
// Module: crate
// Provides: {"warn_or_err"}
// Dependencies: {}
# [doc = " Displays a warning or error (depending on whether warnings are denied)."] # [macro_export] macro_rules ! warn_or_err { ($ diag : expr , $ ($ arg : tt) *) => { $ diag . warn_or_err (format_args ! ($ ($ arg) *)) ; } ; }
};
}
