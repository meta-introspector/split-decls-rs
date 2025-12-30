// Generated macro for check_compare (function)
macro_rules! Depcrate_operators_bit_maskcheck_compare {
() => {
// Module: crate::operators::bit_mask
// Provides: {"check_compare"}
// Dependencies: {}
fn check_compare < 'a > (cx : & LateContext < 'a > , bit_op : & Expr < 'a > , cmp_op : BinOpKind , cmp_value : u128 , span : Span) { if let ExprKind :: Binary (op , left , right) = & bit_op . kind { if op . node != BinOpKind :: BitAnd && op . node != BinOpKind :: BitOr || is_from_proc_macro (cx , bit_op) { return ; } if let Some (mask) = fetch_int_literal (cx , right) . or_else (| | fetch_int_literal (cx , left)) { check_bit_mask (cx , op . node , cmp_op , mask , cmp_value , span) ; } } }
};
}
