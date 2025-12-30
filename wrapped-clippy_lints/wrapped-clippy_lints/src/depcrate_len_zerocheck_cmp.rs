// Generated macro for check_cmp (function)
macro_rules! Depcrate_len_zerocheck_cmp {
() => {
// Module: crate::len_zero
// Provides: {"check_cmp"}
// Dependencies: {}
fn check_cmp (cx : & LateContext < '_ > , span : Span , method : & Expr < '_ > , lit : & Expr < '_ > , op : & str , compare_to : u32) { if method . span . from_expansion () { return ; } if let (& ExprKind :: MethodCall (method_path , receiver , [] , _) , ExprKind :: Lit (lit)) = (& method . kind , & lit . kind) { if parent_item_name (cx , method) == Some (sym :: is_empty) { return ; } check_len (cx , span , method_path . ident . name , receiver , & lit . node , op , compare_to) ; } else { check_empty_expr (cx , span , method , lit , op) ; } }
};
}
