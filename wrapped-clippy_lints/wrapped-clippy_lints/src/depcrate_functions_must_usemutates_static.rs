// Generated macro for mutates_static (function)
macro_rules! Depcrate_functions_must_usemutates_static {
() => {
// Module: crate::functions::must_use
// Provides: {"mutates_static"}
// Dependencies: {}
fn mutates_static < 'tcx > (cx : & LateContext < 'tcx > , body : & 'tcx hir :: Body < '_ >) -> bool { for_each_expr_without_closures (body . value , | e | { use hir :: ExprKind :: { AddrOf , Assign , AssignOp , Call , MethodCall } ; match e . kind { Call (_ , args) => { let mut tys = DefIdSet :: default () ; for arg in args { if cx . tcx . has_typeck_results (arg . hir_id . owner . def_id) && is_mutable_ty (cx , cx . tcx . typeck (arg . hir_id . owner . def_id) . expr_ty (arg) , & mut tys) && is_mutated_static (arg) { return ControlFlow :: Break (()) ; } tys . clear () ; } ControlFlow :: Continue (()) } , MethodCall (_ , receiver , args , _) => { let mut tys = DefIdSet :: default () ; for arg in std :: iter :: once (receiver) . chain (args . iter ()) { if cx . tcx . has_typeck_results (arg . hir_id . owner . def_id) && is_mutable_ty (cx , cx . tcx . typeck (arg . hir_id . owner . def_id) . expr_ty (arg) , & mut tys) && is_mutated_static (arg) { return ControlFlow :: Break (()) ; } tys . clear () ; } ControlFlow :: Continue (()) } , Assign (target , ..) | AssignOp (_ , target , _) | AddrOf (_ , hir :: Mutability :: Mut , target) if is_mutated_static (target) => { ControlFlow :: Break (()) } , _ => ControlFlow :: Continue (()) , } }) . is_some () }
};
}
