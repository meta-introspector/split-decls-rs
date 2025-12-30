// Generated macro for get_arg_root (function)
macro_rules! Depcrate_methods_expect_fun_callget_arg_root {
() => {
// Module: crate::methods::expect_fun_call
// Provides: {"get_arg_root"}
// Dependencies: {}
# [doc = " Strip `{}`, `&`, `as_ref()` and `as_str()` off `arg` until we're left with either a `String` or"] # [doc = " `&str`"] fn get_arg_root < 'a > (cx : & LateContext < '_ > , arg : & 'a hir :: Expr < 'a >) -> & 'a hir :: Expr < 'a > { let mut arg_root = peel_blocks (arg) ; loop { arg_root = match & arg_root . kind { hir :: ExprKind :: AddrOf (hir :: BorrowKind :: Ref , _ , expr) => expr , hir :: ExprKind :: MethodCall (method_name , receiver , [] , ..) => { if (method_name . ident . name == sym :: as_str || method_name . ident . name == sym :: as_ref) && { let arg_type = cx . typeck_results () . expr_ty (receiver) ; let base_type = arg_type . peel_refs () ; base_type . is_str () || base_type . is_lang_item (cx , hir :: LangItem :: String) } { receiver } else { break ; } } , _ => break , } ; } arg_root }
};
}
