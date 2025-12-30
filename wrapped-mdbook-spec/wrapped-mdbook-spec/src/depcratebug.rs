// Generated macro for bug (macro)
macro_rules! Depcratebug {
() => {
// Module: crate
// Provides: {"bug"}
// Dependencies: {}
# [doc = " Displays a message for an internal error, and immediately exits."] # [macro_export] macro_rules ! bug { ($ ($ arg : tt) *) => { eprintln ! ("mdbook-spec internal error: {}" , format_args ! ($ ($ arg) *)) ; std :: process :: exit (1) ; } ; }
};
}
