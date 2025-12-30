// Generated macro for is_body_identity_function (function)
macro_rules! Depcrateis_body_identity_function {
() => {
// Module: crate
// Provides: {"is_body_identity_function"}
// Dependencies: {}
# [doc = " Checks if a function's body represents the identity function. Looks for bodies of the form:"] # [doc = " * `|x| x`"] # [doc = " * `|x| return x`"] # [doc = " * `|x| { return x }`"] # [doc = " * `|x| { return x; }`"] # [doc = " * `|(x, y)| (x, y)`"] # [doc = " * `|[x, y]| [x, y]`"] # [doc = " * `|Foo(bar, baz)| Foo(bar, baz)`"] # [doc = " * `|Foo { bar, baz }| Foo { bar, baz }`"] # [doc = ""] # [doc = " Consider calling [`is_expr_untyped_identity_function`] or [`is_expr_identity_function`] instead."] fn is_body_identity_function (cx : & LateContext < '_ > , func : & Body < '_ >) -> bool { let [param] = func . params else { return false ; } ; let mut expr = func . value ; loop { match expr . kind { ExprKind :: Block (& Block { stmts : [] , expr : Some (e) , .. } , _ ,) | ExprKind :: Ret (Some (e)) => expr = e , ExprKind :: Block (& Block { stmts : [stmt] , expr : None , .. } , _ ,) => { if let StmtKind :: Semi (e) | StmtKind :: Expr (e) = stmt . kind && let ExprKind :: Ret (Some (ret_val)) = e . kind { expr = ret_val ; } else { return false ; } } , _ => return is_expr_identity_of_pat (cx , param . pat , expr , true) , } } }
};
}
