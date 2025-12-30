// Generated macro for InitError (struct)
macro_rules! Depcrate_builtinInitError {
() => {
// Module: crate::builtin
// Provides: {"InitError"}
// Dependencies: {}
# [doc = " Information about why a type cannot be initialized this way."] pub struct InitError { pub (crate) message : String , # [doc = " Spans from struct fields and similar that can be obtained from just the type."] pub (crate) span : Option < Span > , # [doc = " Used to report a trace through adts."] pub (crate) nested : Option < Box < InitError > > , }
};
}
