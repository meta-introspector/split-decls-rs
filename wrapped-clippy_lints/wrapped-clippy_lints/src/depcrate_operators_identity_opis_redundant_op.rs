// Generated macro for is_redundant_op (function)
macro_rules! Depcrate_operators_identity_opis_redundant_op {
() => {
// Module: crate::operators::identity_op
// Provides: {"is_redundant_op"}
// Dependencies: {}
fn is_redundant_op (cx : & LateContext < '_ > , e : & Expr < '_ > , m : i8 , ctxt : SyntaxContext) -> bool { if let Some (Constant :: Int (v)) = ConstEvalCtxt :: new (cx) . eval_local (e , ctxt) . map (Constant :: peel_refs) { let check = match * cx . typeck_results () . expr_ty (e) . peel_refs () . kind () { ty :: Int (ity) => unsext (cx . tcx , - 1_i128 , ity) , ty :: Uint (uty) => clip (cx . tcx , ! 0 , uty) , _ => return false , } ; if match m { 0 => v == 0 , - 1 => v == check , 1 => v == 1 , _ => unreachable ! () , } { return true ; } } false }
};
}
