// Generated macro for check (function)
macro_rules! Depcrate_operators_bit_maskcheck {
() => {
// Module: crate::operators::bit_mask
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , e : & 'tcx Expr < '_ > , op : BinOpKind , left : & 'tcx Expr < '_ > , right : & 'tcx Expr < '_ > ,) { if op . is_comparison () { if let Some (cmp_opt) = fetch_int_literal (cx , right) { check_compare (cx , left , op , cmp_opt , e . span) ; } else if let Some (cmp_val) = fetch_int_literal (cx , left) { check_compare (cx , right , invert_cmp (op) , cmp_val , e . span) ; } } }
};
}
