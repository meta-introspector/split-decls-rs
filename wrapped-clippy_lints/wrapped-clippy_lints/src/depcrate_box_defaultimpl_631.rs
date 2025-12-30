// Generated macro for impl_631 (impl)
macro_rules! Depcrate_box_defaultimpl_631 {
() => {
// Module: crate::box_default
// Provides: {"impl_631"}
// Dependencies: {}
impl Visitor < '_ > for InferVisitor { fn visit_infer (& mut self , inf_id : HirId , _inf_span : Span , _kind : InferKind < '_ >) -> Self :: Result { self . 0 = true ; self . visit_id (inf_id) ; } fn visit_ty (& mut self , t : & Ty < '_ , AmbigArg >) { self . 0 |= matches ! (t . kind , TyKind :: OpaqueDef (..) | TyKind :: TraitObject (..)) ; if ! self . 0 { walk_ty (self , t) ; } } }
};
}
