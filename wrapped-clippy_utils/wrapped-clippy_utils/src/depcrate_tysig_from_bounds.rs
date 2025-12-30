// Generated macro for sig_from_bounds (function)
macro_rules! Depcrate_tysig_from_bounds {
() => {
// Module: crate::ty
// Provides: {"sig_from_bounds"}
// Dependencies: {}
fn sig_from_bounds < 'tcx > (cx : & LateContext < 'tcx > , ty : Ty < 'tcx > , predicates : impl IntoIterator < Item = ty :: Clause < 'tcx > > , predicates_id : Option < DefId > ,) -> Option < ExprFnSig < 'tcx > > { let mut inputs = None ; let mut output = None ; let lang_items = cx . tcx . lang_items () ; for pred in predicates { match pred . kind () . skip_binder () { ty :: ClauseKind :: Trait (p) if (lang_items . fn_trait () == Some (p . def_id ()) || lang_items . fn_mut_trait () == Some (p . def_id ()) || lang_items . fn_once_trait () == Some (p . def_id ())) && p . self_ty () == ty => { let i = pred . kind () . rebind (p . trait_ref . args . type_at (1)) ; if inputs . is_some_and (| inputs | i != inputs) { return None ; } inputs = Some (i) ; } , ty :: ClauseKind :: Projection (p) if Some (p . projection_term . def_id) == lang_items . fn_once_output () && p . projection_term . self_ty () == ty => { if output . is_some () { return None ; } output = Some (pred . kind () . rebind (p . term . expect_type ())) ; } , _ => () , } } inputs . map (| ty | ExprFnSig :: Trait (ty , output , predicates_id)) }
};
}
