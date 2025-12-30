// Generated macro for method_calls (function)
macro_rules! Depcratemethod_calls {
() => {
// Module: crate
// Provides: {"method_calls"}
// Dependencies: {}
# [doc = " Returns the method names and argument list of nested method call expressions that make up"] # [doc = " `expr`. method/span lists are sorted with the most recent call first."] pub fn method_calls < 'tcx > (expr : & 'tcx Expr < 'tcx > , max_depth : usize) -> (Vec < Symbol > , MethodArguments < 'tcx > , Vec < Span >) { let mut method_names = Vec :: with_capacity (max_depth) ; let mut arg_lists = Vec :: with_capacity (max_depth) ; let mut spans = Vec :: with_capacity (max_depth) ; let mut current = expr ; for _ in 0 .. max_depth { if let ExprKind :: MethodCall (path , receiver , args , _) = & current . kind { if receiver . span . from_expansion () || args . iter () . any (| e | e . span . from_expansion ()) { break ; } method_names . push (path . ident . name) ; arg_lists . push ((* receiver , & * * args)) ; spans . push (path . ident . span) ; current = receiver ; } else { break ; } } (method_names , arg_lists , spans) }
};
}
