// Generated macro for check_use (function)
macro_rules! Depcrate_methods_manual_inspectcheck_use {
() => {
// Module: crate::methods::manual_inspect
// Provides: {"check_use"}
// Dependencies: {}
# [doc = " Checks how the value is used, and whether it was used in the same `SyntaxContext`."] fn check_use < 'tcx > (cx : & LateContext < 'tcx > , e : & 'tcx Expr < '_ >) -> (UseKind < 'tcx > , bool) { let use_cx = expr_use_ctxt (cx , e) ; if use_cx . adjustments . first () . is_some_and (| a | matches ! (a . kind , Adjust :: Deref (_))) { return (UseKind :: AutoBorrowed , use_cx . same_ctxt) ; } let res = match use_cx . use_node (cx) { ExprUseNode :: Return (_) => { if let ExprKind :: Ret (Some (e)) = use_cx . node . expect_expr () . kind { UseKind :: Return (e . span) } else { return (UseKind :: Return (DUMMY_SP) , false) ; } } , ExprUseNode :: FieldAccess (name) => UseKind :: FieldAccess (name . name , use_cx . node . expect_expr ()) , ExprUseNode :: Callee | ExprUseNode :: MethodArg (_ , _ , 0) if use_cx . adjustments . first () . is_some_and (| a | matches ! (a . kind , Adjust :: Borrow (AutoBorrow :: Ref (AutoBorrowMutability :: Not)))) => { UseKind :: AutoBorrowed } , ExprUseNode :: Callee | ExprUseNode :: MethodArg (_ , _ , 0) => UseKind :: WillAutoDeref , ExprUseNode :: AddrOf (BorrowKind :: Ref , _) => UseKind :: Borrowed (use_cx . node . expect_expr () . span) , _ => UseKind :: Deref , } ; (res , use_cx . same_ctxt) }
};
}
