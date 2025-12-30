// Generated macro for extract_anonymous_ref (function)
macro_rules! Depcrate_unnecessary_literal_boundextract_anonymous_ref {
() => {
// Module: crate::unnecessary_literal_bound
// Provides: {"extract_anonymous_ref"}
// Dependencies: {}
fn extract_anonymous_ref < 'tcx > (hir_ty : & Ty < 'tcx >) -> Option < & 'tcx Ty < 'tcx > > { let TyKind :: Ref (lifetime , MutTy { ty , mutbl }) = hir_ty . kind else { return None ; } ; if ! lifetime . is_anonymous () || ! matches ! (mutbl , Mutability :: Not) { return None ; } Some (ty) }
};
}
