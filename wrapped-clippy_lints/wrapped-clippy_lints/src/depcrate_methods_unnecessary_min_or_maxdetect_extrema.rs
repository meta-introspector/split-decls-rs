// Generated macro for detect_extrema (function)
macro_rules! Depcrate_methods_unnecessary_min_or_maxdetect_extrema {
() => {
// Module: crate::methods::unnecessary_min_or_max
// Provides: {"detect_extrema"}
// Dependencies: {}
fn detect_extrema < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) -> Option < Extrema > { let ty = cx . typeck_results () . expr_ty (expr) ; let cv = ConstEvalCtxt :: new (cx) . eval (expr) ? ; match (cv . int_value (cx . tcx , ty) ? , ty . kind ()) { (FullInt :: S (i) , & ty :: Int (ity)) if i == i128 :: MIN >> (128 - ity . bit_width () ?) => Some (Extrema :: Minimum) , (FullInt :: S (i) , & ty :: Int (ity)) if i == i128 :: MAX >> (128 - ity . bit_width () ?) => Some (Extrema :: Maximum) , (FullInt :: U (i) , & ty :: Uint (uty)) if i == u128 :: MAX >> (128 - uty . bit_width () ?) => Some (Extrema :: Maximum) , (FullInt :: U (0) , & ty :: Uint (_)) => Some (Extrema :: Minimum) , _ => None , } }
};
}
