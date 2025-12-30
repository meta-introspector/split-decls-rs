// Generated macro for type_certainty (function)
macro_rules! Depcrate_ty_type_certaintytype_certainty {
() => {
// Module: crate::ty::type_certainty
// Provides: {"type_certainty"}
// Dependencies: {}
fn type_certainty (cx : & LateContext < '_ > , ty : & hir :: Ty < '_ >) -> Certainty { if let TyKind :: Path (qpath) = & ty . kind { return qpath_certainty (cx , qpath , true) ; } let mut visitor = CertaintyVisitor :: new (cx) ; visitor . visit_ty_unambig (ty) ; visitor . certainty }
};
}
