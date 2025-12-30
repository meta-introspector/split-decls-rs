// Generated macro for skip_addr_of_ancestors (function)
macro_rules! Depcrate_methods_unnecessary_to_ownedskip_addr_of_ancestors {
() => {
// Module: crate::methods::unnecessary_to_owned
// Provides: {"skip_addr_of_ancestors"}
// Dependencies: {}
# [doc = " Walks an expression's ancestors until it finds a non-`AddrOf` expression. Returns the first such"] # [doc = " expression found (if any) along with the immediately prior expression."] fn skip_addr_of_ancestors < 'tcx > (cx : & LateContext < 'tcx > , mut expr : & 'tcx Expr < 'tcx > ,) -> Option < (& 'tcx Expr < 'tcx > , & 'tcx Expr < 'tcx >) > { while let Some (parent) = get_parent_expr (cx , expr) { if let ExprKind :: AddrOf (BorrowKind :: Ref , Mutability :: Not , _) = parent . kind { expr = parent ; } else { return Some ((parent , expr)) ; } } None }
};
}
