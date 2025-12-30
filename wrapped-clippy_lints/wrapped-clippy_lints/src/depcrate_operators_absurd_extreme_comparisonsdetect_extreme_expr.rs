// Generated macro for detect_extreme_expr (function)
macro_rules! Depcrate_operators_absurd_extreme_comparisonsdetect_extreme_expr {
() => {
// Module: crate::operators::absurd_extreme_comparisons
// Provides: {"detect_extreme_expr"}
// Dependencies: {}
fn detect_extreme_expr < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) -> Option < ExtremeExpr < 'tcx > > { let ty = cx . typeck_results () . expr_ty (expr) ; let cv = ConstEvalCtxt :: new (cx) . eval (expr) ? ; let which = match (ty . kind () , cv) { (& ty :: Bool , Constant :: Bool (false)) | (& ty :: Uint (_) , Constant :: Int (0)) => ExtremeType :: Minimum , (& ty :: Int (ity) , Constant :: Int (i)) if i == unsext (cx . tcx , i128 :: MIN >> (128 - int_bits (cx . tcx , ity)) , ity) => { ExtremeType :: Minimum } , (& ty :: Bool , Constant :: Bool (true)) => ExtremeType :: Maximum , (& ty :: Int (ity) , Constant :: Int (i)) if i == unsext (cx . tcx , i128 :: MAX >> (128 - int_bits (cx . tcx , ity)) , ity) => { ExtremeType :: Maximum } , (& ty :: Uint (uty) , Constant :: Int (i)) if clip (cx . tcx , u128 :: MAX , uty) == i => ExtremeType :: Maximum , _ => return None , } ; Some (ExtremeExpr { which , expr }) }
};
}
