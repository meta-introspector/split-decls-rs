// Generated macro for sig_for_projection (function)
macro_rules! Depcrate_tysig_for_projection {
() => {
// Module: crate::ty
// Provides: {"sig_for_projection"}
// Dependencies: {}
fn sig_for_projection < 'tcx > (cx : & LateContext < 'tcx > , ty : AliasTy < 'tcx >) -> Option < ExprFnSig < 'tcx > > { let mut inputs = None ; let mut output = None ; let lang_items = cx . tcx . lang_items () ; for (pred , _) in cx . tcx . explicit_item_bounds (ty . def_id) . iter_instantiated_copied (cx . tcx , ty . args) { match pred . kind () . skip_binder () { ty :: ClauseKind :: Trait (p) if (lang_items . fn_trait () == Some (p . def_id ()) || lang_items . fn_mut_trait () == Some (p . def_id ()) || lang_items . fn_once_trait () == Some (p . def_id ())) => { let i = pred . kind () . rebind (p . trait_ref . args . type_at (1)) ; if inputs . is_some_and (| inputs | inputs != i) { return None ; } inputs = Some (i) ; } , ty :: ClauseKind :: Projection (p) if Some (p . projection_term . def_id) == lang_items . fn_once_output () => { if output . is_some () { return None ; } output = pred . kind () . rebind (p . term . as_type ()) . transpose () ; } , _ => () , } } inputs . map (| ty | ExprFnSig :: Trait (ty , output , None)) }
};
}
