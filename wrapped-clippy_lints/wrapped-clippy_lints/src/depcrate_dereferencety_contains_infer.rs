// Generated macro for ty_contains_infer (function)
macro_rules! Depcrate_dereferencety_contains_infer {
() => {
// Module: crate::dereference
// Provides: {"ty_contains_infer"}
// Dependencies: {}
fn ty_contains_infer (ty : & hir :: Ty < '_ >) -> bool { struct V (bool) ; impl Visitor < '_ > for V { fn visit_infer (& mut self , inf_id : HirId , _inf_span : Span , kind : InferKind < '_ >) -> Self :: Result { if let InferKind :: Ty (_) | InferKind :: Ambig (_) = kind { self . 0 = true ; } self . visit_id (inf_id) ; } fn visit_ty (& mut self , ty : & hir :: Ty < '_ , AmbigArg >) { if self . 0 || matches ! (ty . kind , TyKind :: OpaqueDef (..) | TyKind :: Typeof (_) | TyKind :: Err (_)) { self . 0 = true ; } else { walk_ty (self , ty) ; } } } let mut v = V (false) ; v . visit_ty_unambig (ty) ; v . 0 }
};
}
