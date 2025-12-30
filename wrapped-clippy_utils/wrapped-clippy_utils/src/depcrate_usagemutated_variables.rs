// Generated macro for mutated_variables (function)
macro_rules! Depcrate_usagemutated_variables {
() => {
// Module: crate::usage
// Provides: {"mutated_variables"}
// Dependencies: {}
# [doc = " Returns a set of mutated local variable IDs, or `None` if mutations could not be determined."] pub fn mutated_variables < 'tcx > (expr : & 'tcx Expr < '_ > , cx : & LateContext < 'tcx >) -> Option < HirIdSet > { let mut delegate = MutVarsDelegate { used_mutably : HirIdSet :: default () , skip : false , } ; ExprUseVisitor :: for_clippy (cx , expr . hir_id . owner . def_id , & mut delegate) . walk_expr (expr) . into_ok () ; if delegate . skip { return None ; } Some (delegate . used_mutably) }
};
}
