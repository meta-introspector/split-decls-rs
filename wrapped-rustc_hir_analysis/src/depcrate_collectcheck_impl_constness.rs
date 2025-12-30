// Generated macro for check_impl_constness (function)
macro_rules! Depcrate_collectcheck_impl_constness {
() => {
// Module: crate::collect
// Provides: {"check_impl_constness"}
// Dependencies: {}
fn check_impl_constness (tcx : TyCtxt < '_ > , constness : hir :: Constness , hir_trait_ref : & hir :: TraitRef < '_ > ,) { if let hir :: Constness :: NotConst = constness { return ; } let Some (trait_def_id) = hir_trait_ref . trait_def_id () else { return } ; if tcx . is_const_trait (trait_def_id) { return ; } let trait_name = tcx . item_name (trait_def_id) . to_string () ; let (local_trait_span , suggestion_pre) = match (trait_def_id . is_local () , tcx . sess . is_nightly_build ()) { (true , true) => (Some (tcx . def_span (trait_def_id) . shrink_to_lo ()) , if tcx . features () . const_trait_impl () { "" } else { "enable `#![feature(const_trait_impl)]` in your crate and " } ,) , (false , _) | (_ , false) => (None , "") , } ; tcx . dcx () . emit_err (errors :: ConstImplForNonConstTrait { trait_ref_span : hir_trait_ref . path . span , trait_name , local_trait_span , suggestion_pre , marking : () , adding : () , }) ; }
};
}
