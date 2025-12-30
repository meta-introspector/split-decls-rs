// Generated macro for is_boolean_literal (function)
macro_rules! Depcrate_parseris_boolean_literal {
() => {
// Module: crate::parser
// Provides: {"is_boolean_literal"}
// Dependencies: {}
fn is_boolean_literal (lit : & tt :: Literal < Span >) -> bool { matches ! (lit . symbol . as_str () , "true" | "false") }
};
}
