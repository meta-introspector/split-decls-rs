// Generated macro for impl_3890 (impl)
macro_rules! Depcrate_loops_while_immutable_conditionimpl_3890 {
() => {
// Module: crate::loops::while_immutable_condition
// Provides: {"impl_3890"}
// Dependencies: {}
impl < 'tcx > VarCollectorVisitor < '_ , 'tcx > { fn insert_def_id (& mut self , ex : & 'tcx Expr < '_ >) { if let ExprKind :: Path (ref qpath) = ex . kind && let QPath :: Resolved (None , _) = * qpath { match self . cx . qpath_res (qpath , ex . hir_id) { Res :: Local (hir_id) => { self . ids . insert (hir_id) ; } , Res :: Def (DefKind :: Static { .. } , def_id) => { let mutable = self . cx . tcx . is_mutable_static (def_id) ; self . def_ids . insert (def_id , mutable) ; } , _ => { } , } } } }
};
}
