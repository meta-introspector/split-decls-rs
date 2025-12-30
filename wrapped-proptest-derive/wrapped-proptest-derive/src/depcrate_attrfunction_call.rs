// Generated macro for function_call (function)
macro_rules! Depcrate_attrfunction_call {
() => {
// Module: crate::attr
// Provides: {"function_call"}
// Dependencies: {}
# [doc = " Construct a function call expression for an identifier."] fn function_call (fun : Ident) -> Expr { parse_quote ! (# fun ()) }
};
}
