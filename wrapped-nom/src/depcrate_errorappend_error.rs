// Generated macro for append_error (function)
macro_rules! Depcrate_errorappend_error {
() => {
// Module: crate::error
// Provides: {"append_error"}
// Dependencies: {}
# [doc = " Combines an existing error with a new one created from the input"] # [doc = " position and an [ErrorKind]. This is useful when backtracking"] # [doc = " through a parse tree, accumulating error context on the way"] pub fn append_error < I , E : ParseError < I > > (input : I , kind : ErrorKind , other : E) -> E { E :: append (input , kind , other) }
};
}
