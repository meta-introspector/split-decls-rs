// Generated macro for impl_54 (impl)
macro_rules! Depcrate_infer_atimpl_54 {
() => {
// Module: crate::infer::at
// Provides: {"impl_54"}
// Dependencies: {}
impl < 'tcx > ToTrace < 'tcx > for ImplSubject < 'tcx > { fn to_trace (cause : & ObligationCause < 'tcx > , a : Self , b : Self) -> TypeTrace < 'tcx > { match (a , b) { (ImplSubject :: Trait (trait_ref_a) , ImplSubject :: Trait (trait_ref_b)) => { ToTrace :: to_trace (cause , trait_ref_a , trait_ref_b) } (ImplSubject :: Inherent (ty_a) , ImplSubject :: Inherent (ty_b)) => { ToTrace :: to_trace (cause , ty_a , ty_b) } (ImplSubject :: Trait (_) , ImplSubject :: Inherent (_)) | (ImplSubject :: Inherent (_) , ImplSubject :: Trait (_)) => { bug ! ("can not trace TraitRef and Ty") ; } } } }
};
}
