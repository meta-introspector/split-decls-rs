// Generated macro for impl_165 (impl)
macro_rules! Depcrate_normalizeimpl_165 {
() => {
// Module: crate::normalize
// Provides: {"impl_165"}
// Dependencies: {}
impl < 'a , S : Spec > From < & 'a RiAbsoluteStr < S > > for NormalizationInput < 'a > { fn from (iri : & 'a RiAbsoluteStr < S >) -> Self { let components = RiReferenceComponents :: < S > :: from (iri . as_ref ()) ; let (scheme , authority , path , query , fragment) = components . to_major () ; let scheme = scheme . expect ("[validity] `absolute IRI must have `scheme`") ; let path = Path :: NeedsProcessing (PathToNormalize :: from_single_path (path)) ; NormalizationInput { scheme , authority , path , query , fragment , op : NormalizationOp { mode : NormalizationMode :: None , } , } } }
};
}
