// Generated macro for build_ref_method_chain (function)
macro_rules! Depcrate_matches_collapsible_matchbuild_ref_method_chain {
() => {
// Module: crate::matches::collapsible_match
// Provides: {"build_ref_method_chain"}
// Dependencies: {}
# [doc = " Builds a chain of reference-manipulation method calls (e.g., `.as_ref()`, `.as_mut()`,"] # [doc = " `.copied()`) based on reference operators"] fn build_ref_method_chain (expr : Vec < & Expr < '_ > >) -> Option < String > { let mut req_method_calls = String :: new () ; for ref_operator in expr { match ref_operator . kind { ExprKind :: AddrOf (BorrowKind :: Raw , _ , _) => { return None ; } , ExprKind :: AddrOf (_ , m , _) if m . is_mut () => { req_method_calls . push_str (".as_mut()") ; } , ExprKind :: AddrOf (_ , _ , _) => { req_method_calls . push_str (".as_ref()") ; } , ExprKind :: Unary (_ , _) => { req_method_calls . push_str (".copied()") ; } , _ => () , } } Some (req_method_calls) }
};
}
