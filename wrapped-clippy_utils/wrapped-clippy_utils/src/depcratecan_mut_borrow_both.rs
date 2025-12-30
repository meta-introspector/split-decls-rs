// Generated macro for can_mut_borrow_both (function)
macro_rules! Depcratecan_mut_borrow_both {
() => {
// Module: crate
// Provides: {"can_mut_borrow_both"}
// Dependencies: {}
# [doc = " Checks if two expressions can be mutably borrowed simultaneously"] # [doc = " and they aren't dependent on borrowing same thing twice"] pub fn can_mut_borrow_both (cx : & LateContext < '_ > , e1 : & Expr < '_ > , e2 : & Expr < '_ >) -> bool { let (s1 , r1) = projection_stack (e1) ; let (s2 , r2) = projection_stack (e2) ; if ! eq_expr_value (cx , r1 , r2) { return true ; } if expr_custom_deref_adjustment (cx , r1) . is_some () || expr_custom_deref_adjustment (cx , r2) . is_some () { return false ; } for (x1 , x2) in zip (& s1 , & s2) { if expr_custom_deref_adjustment (cx , x1) . is_some () || expr_custom_deref_adjustment (cx , x2) . is_some () { return false ; } match (& x1 . kind , & x2 . kind) { (ExprKind :: Field (_ , i1) , ExprKind :: Field (_ , i2)) => { if i1 != i2 { return true ; } } , (ExprKind :: Index (_ , i1 , _) , ExprKind :: Index (_ , i2 , _)) => { if ! eq_expr_value (cx , i1 , i2) { return false ; } } , _ => return false , } } false }
};
}
