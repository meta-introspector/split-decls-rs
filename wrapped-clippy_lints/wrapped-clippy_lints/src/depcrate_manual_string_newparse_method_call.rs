// Generated macro for parse_method_call (function)
macro_rules! Depcrate_manual_string_newparse_method_call {
() => {
// Module: crate::manual_string_new
// Provides: {"parse_method_call"}
// Dependencies: {}
# [doc = " Tries to parse an expression as a method call, emitting the warning if necessary."] fn parse_method_call (cx : & LateContext < '_ > , span : Span , path_segment : & PathSegment < '_ > , receiver : & Expr < '_ >) { let method_arg_kind = & receiver . kind ; if matches ! (path_segment . ident . name , sym :: to_string | sym :: to_owned | sym :: into) && is_expr_kind_empty_str (method_arg_kind) { warn_then_suggest (cx , span) ; } else if let ExprKind :: Call (func , [arg]) = method_arg_kind { parse_call (cx , span , func , arg) ; } }
};
}
