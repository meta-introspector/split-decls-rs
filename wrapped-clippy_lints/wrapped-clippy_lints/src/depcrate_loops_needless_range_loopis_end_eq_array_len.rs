// Generated macro for is_end_eq_array_len (function)
macro_rules! Depcrate_loops_needless_range_loopis_end_eq_array_len {
() => {
// Module: crate::loops::needless_range_loop
// Provides: {"is_end_eq_array_len"}
// Dependencies: {}
fn is_end_eq_array_len < 'tcx > (cx : & LateContext < 'tcx > , end : & Expr < '_ > , limits : ast :: RangeLimits , indexed_ty : Ty < 'tcx > ,) -> bool { if let ExprKind :: Lit (lit) = end . kind && let ast :: LitKind :: Int (end_int , _) = lit . node && let ty :: Array (_ , arr_len_const) = indexed_ty . kind () && let Some (arr_len) = arr_len_const . try_to_target_usize (cx . tcx) { return match limits { ast :: RangeLimits :: Closed => end_int . get () + 1 >= arr_len . into () , ast :: RangeLimits :: HalfOpen => end_int . get () >= arr_len . into () , } ; } false }
};
}
