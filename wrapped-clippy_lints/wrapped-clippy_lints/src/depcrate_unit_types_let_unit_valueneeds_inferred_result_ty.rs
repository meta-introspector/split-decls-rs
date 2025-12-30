// Generated macro for needs_inferred_result_ty (function)
macro_rules! Depcrate_unit_types_let_unit_valueneeds_inferred_result_ty {
() => {
// Module: crate::unit_types::let_unit_value
// Provides: {"needs_inferred_result_ty"}
// Dependencies: {}
fn needs_inferred_result_ty (cx : & LateContext < '_ > , e : & Expr < '_ > , locals_to_check : & mut Vec < HirId > , seen_locals : & mut HirIdSet ,) -> bool { let (id , receiver , args) = match e . kind { ExprKind :: Call (Expr { kind : ExprKind :: Path (path) , hir_id , .. } , args ,) => match cx . qpath_res (path , * hir_id) { Res :: Def (DefKind :: AssocFn | DefKind :: Fn , id) => (id , None , args) , _ => return false , } , ExprKind :: MethodCall (_ , receiver , args , _) => match cx . typeck_results () . type_dependent_def_id (e . hir_id) { Some (id) => (id , Some (receiver) , args) , None => return false , } , ExprKind :: Path (QPath :: Resolved (None , path)) => { if let Res :: Local (id) = path . res && seen_locals . insert (id) { locals_to_check . push (id) ; } return true ; } , _ => return false , } ; let sig = cx . tcx . fn_sig (id) . instantiate_identity () . skip_binder () ; if let ty :: Param (output_ty) = * sig . output () . kind () { let args : Vec < & Expr < '_ > > = if let Some (receiver) = receiver { std :: iter :: once (receiver) . chain (args . iter ()) . collect () } else { args . iter () . collect () } ; sig . inputs () . iter () . zip (args) . all (| (& ty , arg) | { ! ty . is_param (output_ty . index) || each_value_source_needs_inference (cx , arg , locals_to_check , seen_locals) }) } else { false } }
};
}
