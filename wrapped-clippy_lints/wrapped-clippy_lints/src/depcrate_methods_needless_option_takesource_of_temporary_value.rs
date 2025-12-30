// Generated macro for source_of_temporary_value (function)
macro_rules! Depcrate_methods_needless_option_takesource_of_temporary_value {
() => {
// Module: crate::methods::needless_option_take
// Provides: {"source_of_temporary_value"}
// Dependencies: {}
# [doc = " Returns the string of the function call that creates the temporary."] # [doc = " When this function is called, we are reasonably certain that the `ExprKind` is either"] # [doc = " `Call` or `MethodCall` because we already checked that the expression is not"] # [doc = " `is_syntactic_place_expr()`."] fn source_of_temporary_value (expr : & Expr < '_ >) -> Option < Symbol > { match expr . peel_borrows () . kind { ExprKind :: Call (function , _) => { if let ExprKind :: Path (QPath :: Resolved (_ , func_path)) = function . kind && ! func_path . segments . is_empty () { return Some (func_path . segments [0] . ident . name) ; } if let ExprKind :: Path (QPath :: TypeRelative (_ , func_path_segment)) = function . kind { return Some (func_path_segment . ident . name) ; } None } , ExprKind :: MethodCall (path_segment , ..) => Some (path_segment . ident . name) , _ => None , } }
};
}
