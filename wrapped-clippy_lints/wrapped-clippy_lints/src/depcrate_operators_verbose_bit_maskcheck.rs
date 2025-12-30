// Generated macro for check (function)
macro_rules! Depcrate_operators_verbose_bit_maskcheck {
() => {
// Module: crate::operators::verbose_bit_mask
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , e : & 'tcx Expr < '_ > , op : BinOpKind , left : & 'tcx Expr < '_ > , right : & 'tcx Expr < '_ > , threshold : u64 ,) { if BinOpKind :: Eq == op && let ExprKind :: Binary (op1 , left1 , right1) = & left . kind && BinOpKind :: BitAnd == op1 . node && let ExprKind :: Lit (lit) = & right1 . kind && let LitKind :: Int (Pu128 (n) , _) = lit . node && let ExprKind :: Lit (lit1) = & right . kind && let LitKind :: Int (Pu128 (0) , _) = lit1 . node && n . leading_zeros () == n . count_zeros () && n > u128 :: from (threshold) { span_lint_and_then (cx , VERBOSE_BIT_MASK , e . span , "bit mask could be simplified with a call to `trailing_zeros`" , | diag | { let sugg = Sugg :: hir (cx , left1 , "...") . maybe_paren () ; diag . span_suggestion (e . span , "try" , format ! ("{sugg}.trailing_zeros() >= {}" , n . count_ones ()) , Applicability :: MaybeIncorrect ,) ; } ,) ; } }
};
}
