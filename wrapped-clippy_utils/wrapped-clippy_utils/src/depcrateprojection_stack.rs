// Generated macro for projection_stack (function)
macro_rules! Depcrateprojection_stack {
() => {
// Module: crate
// Provides: {"projection_stack"}
// Dependencies: {}
# [doc = " This method will return tuple of projection stack and root of the expression,"] # [doc = " used in `can_mut_borrow_both`."] # [doc = ""] # [doc = " For example, if `e` represents the `v[0].a.b[x]`"] # [doc = " this method will return a tuple, composed of a `Vec`"] # [doc = " containing the `Expr`s for `v[0], v[0].a, v[0].a.b, v[0].a.b[x]`"] # [doc = " and an `Expr` for root of them, `v`"] fn projection_stack < 'a , 'hir > (mut e : & 'a Expr < 'hir >) -> (Vec < & 'a Expr < 'hir > > , & 'a Expr < 'hir >) { let mut result = vec ! [] ; let root = loop { match e . kind { ExprKind :: Index (ep , _ , _) | ExprKind :: Field (ep , _) => { result . push (e) ; e = ep ; } , _ => break e , } } ; result . reverse () ; (result , root) }
};
}
