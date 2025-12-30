// Generated macro for impl_9541 (impl)
macro_rules! Depcrate_replace_boximpl_9541 {
() => {
// Module: crate::replace_box
// Provides: {"impl_9541"}
// Dependencies: {}
impl ReplaceBox { fn get_consumed_locals (& mut self , cx : & LateContext < '_ >) -> & FxHashSet < HirId > { if let Some (body_id) = cx . enclosing_body && ! self . loaded_bodies . contains (& body_id) { self . loaded_bodies . push (body_id) ; ExprUseVisitor :: for_clippy (cx , cx . tcx . hir_body_owner_def_id (body_id) , MovedVariablesCtxt { consumed_locals : & mut self . consumed_locals , } ,) . consume_body (cx . tcx . hir_body (body_id)) . into_ok () ; } & self . consumed_locals } }
};
}
