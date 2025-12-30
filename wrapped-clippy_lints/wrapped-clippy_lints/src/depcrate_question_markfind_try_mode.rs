// Generated macro for find_try_mode (function)
macro_rules! Depcrate_question_markfind_try_mode {
() => {
// Module: crate::question_mark
// Provides: {"find_try_mode"}
// Dependencies: {}
fn find_try_mode < 'tcx > (cx : & LateContext < 'tcx > , scrutinee : & Expr < 'tcx >) -> Option < TryMode > { let scrutinee_ty = cx . typeck_results () . expr_ty_adjusted (scrutinee) ; let ty :: Adt (scrutinee_adt_def , _) = scrutinee_ty . kind () else { return None ; } ; match cx . tcx . get_diagnostic_name (scrutinee_adt_def . did ()) ? { sym :: Result => Some (TryMode :: Result) , sym :: Option => Some (TryMode :: Option) , _ => None , } }
};
}
