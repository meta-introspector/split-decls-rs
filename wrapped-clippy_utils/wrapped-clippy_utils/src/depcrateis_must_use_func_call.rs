// Generated macro for is_must_use_func_call (function)
macro_rules! Depcrateis_must_use_func_call {
() => {
// Module: crate
// Provides: {"is_must_use_func_call"}
// Dependencies: {}
pub fn is_must_use_func_call (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> bool { let did = match expr . kind { ExprKind :: Call (path , _) => { if let ExprKind :: Path (ref qpath) = path . kind && let Res :: Def (_ , did) = cx . qpath_res (qpath , path . hir_id) { Some (did) } else { None } } , ExprKind :: MethodCall (..) => cx . typeck_results () . type_dependent_def_id (expr . hir_id) , _ => None , } ; did . is_some_and (| did | find_attr ! (cx . tcx . get_all_attrs (did) , AttributeKind :: MustUse { .. })) }
};
}
