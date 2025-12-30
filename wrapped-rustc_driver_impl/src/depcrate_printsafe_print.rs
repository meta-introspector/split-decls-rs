// Generated macro for safe_print (macro)
macro_rules! Depcrate_printsafe_print {
() => {
// Module: crate::print
// Provides: {"safe_print"}
// Dependencies: {}
macro_rules ! safe_print { ($ ($ arg : tt) *) => { { $ crate :: print :: print (std :: format_args ! ($ ($ arg) *)) ; } } ; }
};
}
