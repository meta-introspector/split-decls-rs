// Generated macro for impl_771 (impl)
macro_rules! Depcrate_ty_type_certaintyimpl_771 {
() => {
// Module: crate::ty::type_certainty
// Provides: {"impl_771"}
// Dependencies: {}
impl < 'cx > Visitor < 'cx > for CertaintyVisitor < 'cx , '_ > { fn visit_qpath (& mut self , qpath : & 'cx QPath < '_ > , hir_id : HirId , _ : Span) { self . certainty = self . certainty . meet (qpath_certainty (self . cx , qpath , true)) ; if self . certainty != Certainty :: Uncertain { walk_qpath (self , qpath , hir_id) ; } } fn visit_ty (& mut self , ty : & 'cx hir :: Ty < '_ , AmbigArg >) { if self . certainty != Certainty :: Uncertain { walk_ty (self , ty) ; } } fn visit_infer (& mut self , _inf_id : HirId , _inf_span : Span , _kind : InferKind < 'cx >) -> Self :: Result { self . certainty = Certainty :: Uncertain ; } }
};
}
