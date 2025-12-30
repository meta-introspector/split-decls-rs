// Generated macro for is_array_length_equal_to_range (function)
macro_rules! Depcrate_loops_manual_memcpyis_array_length_equal_to_range {
() => {
// Module: crate::loops::manual_memcpy
// Provides: {"is_array_length_equal_to_range"}
// Dependencies: {}
fn is_array_length_equal_to_range (cx : & LateContext < '_ > , start : & Expr < '_ > , end : & Expr < '_ > , arr : & Expr < '_ >) -> bool { fn extract_lit_value (expr : & Expr < '_ >) -> Option < u128 > { if let ExprKind :: Lit (lit) = expr . kind && let ast :: LitKind :: Int (value , _) = lit . node { Some (value . get ()) } else { None } } let arr_ty = cx . typeck_results () . expr_ty (arr) . peel_refs () ; if let ty :: Array (_ , s) = arr_ty . kind () { let size : u128 = if let Some (size) = s . try_to_target_usize (cx . tcx) { size . into () } else { return false ; } ; let range = match (extract_lit_value (start) , extract_lit_value (end)) { (Some (start_value) , Some (end_value)) => end_value - start_value , _ => return false , } ; size == range } else { false } }
};
}
