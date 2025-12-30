// Generated macro for Repr (enum)
macro_rules! Depcrate_errorRepr {
() => {
// Module: crate::error
// Provides: {"Repr"}
// Dependencies: {}
# [derive (Debug)] enum Repr { Single { text : String , span : Option < (Span , Span) > , } , SynError (Error) , Multi { diagnostics : Vec < Diagnostic > , } , }
};
}
