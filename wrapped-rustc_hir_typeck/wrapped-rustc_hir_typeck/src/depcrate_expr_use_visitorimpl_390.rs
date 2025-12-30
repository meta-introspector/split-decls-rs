// Generated macro for impl_390 (impl)
macro_rules! Depcrate_expr_use_visitorimpl_390 {
() => {
// Module: crate::expr_use_visitor
// Provides: {"impl_390"}
// Dependencies: {}
impl < 'a , 'tcx , D : Delegate < 'tcx > > ExprUseVisitor < 'tcx , (& 'a LateContext < 'tcx > , LocalDefId) , D > { pub fn for_clippy (cx : & 'a LateContext < 'tcx > , body_def_id : LocalDefId , delegate : D) -> Self { Self :: new ((cx , body_def_id) , delegate) } }
};
}
