// Generated macro for trace_init (macro)
macro_rules! Depcratetrace_init {
() => {
// Module: crate
// Provides: {"trace_init"}
// Dependencies: {}
# [macro_export] macro_rules ! trace_init { () => { let _guard = $ crate :: trace :: init () ; let span = $ crate :: prelude :: tracing :: info_span ! ("test" , "{}" , std :: thread :: current () . name () . expect ("test threads must be named")) ; let _e = span . enter () ; } ; }
};
}
