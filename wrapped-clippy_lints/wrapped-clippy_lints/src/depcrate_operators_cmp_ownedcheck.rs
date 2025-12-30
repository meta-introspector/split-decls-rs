// Generated macro for check (function)
macro_rules! Depcrate_operators_cmp_ownedcheck {
() => {
// Module: crate::operators::cmp_owned
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , op : BinOpKind , lhs : & Expr < '_ > , rhs : & Expr < '_ >) { if op . is_comparison () { check_op (cx , lhs , rhs , true) ; check_op (cx , rhs , lhs , false) ; } }
};
}
