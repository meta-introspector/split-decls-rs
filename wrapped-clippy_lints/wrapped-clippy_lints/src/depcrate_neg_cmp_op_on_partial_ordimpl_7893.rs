// Generated macro for impl_7893 (impl)
macro_rules! Depcrate_neg_cmp_op_on_partial_ordimpl_7893 {
() => {
// Module: crate::neg_cmp_op_on_partial_ord
// Provides: {"impl_7893"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for NoNegCompOpForPartialOrd { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) { if let ExprKind :: Unary (UnOp :: Not , inner) = expr . kind && let ExprKind :: Binary (ref op , left , _) = inner . kind && let BinOpKind :: Le | BinOpKind :: Ge | BinOpKind :: Lt | BinOpKind :: Gt = op . node && ! expr . span . in_external_macro (cx . sess () . source_map ()) { let ty = cx . typeck_results () . expr_ty (left) ; let implements_ord = { if let Some (id) = cx . tcx . get_diagnostic_item (sym :: Ord) { implements_trait (cx , ty , id , & []) } else { return ; } } ; let implements_partial_ord = { if let Some (id) = cx . tcx . lang_items () . partial_ord_trait () { implements_trait (cx , ty , id , & [ty . into ()]) } else { return ; } } ; if implements_partial_ord && ! implements_ord { span_lint (cx , NEG_CMP_OP_ON_PARTIAL_ORD , expr . span , "the use of negated comparison operators on partially ordered \
                    types produces code that is hard to read and refactor, please \
                    consider using the `partial_cmp` method instead, to make it \
                    clear that the two values could be incomparable" ,) ; } } } }
};
}
