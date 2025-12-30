// Generated macro for safe_println (macro)
macro_rules! Depcrate_printsafe_println {
() => {
// Module: crate::print
// Provides: {"safe_println"}
// Dependencies: {}
macro_rules ! safe_println { ($ ($ arg : tt) *) => { safe_print ! ("{}\n" , std :: format_args ! ($ ($ arg) *)) } ; }
};
}
