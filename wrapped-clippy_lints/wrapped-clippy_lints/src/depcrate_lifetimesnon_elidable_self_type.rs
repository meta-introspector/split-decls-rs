// Generated macro for non_elidable_self_type (function)
macro_rules! Depcrate_lifetimesnon_elidable_self_type {
() => {
// Module: crate::lifetimes
// Provides: {"non_elidable_self_type"}
// Dependencies: {}
fn non_elidable_self_type < 'tcx > (cx : & LateContext < 'tcx > , func : & FnDecl < 'tcx > , ident : Option < Ident > , msrv : Msrv) -> bool { if let Some (ident) = ident && ident . name == kw :: SelfLower && ! func . implicit_self . has_implicit_self () && let Some (self_ty) = func . inputs . first () && ! msrv . meets (cx , msrvs :: EXPLICIT_SELF_TYPE_ELISION) { let mut visitor = RefVisitor :: new (cx) ; visitor . visit_ty_unambig (self_ty) ; ! visitor . all_lts () . is_empty () } else { false } }
};
}
