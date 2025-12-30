// Generated macro for impl_163 (impl)
macro_rules! Depcrate_normalizeimpl_163 {
() => {
// Module: crate::normalize
// Provides: {"impl_163"}
// Dependencies: {}
impl < 'a , S : Spec > From < & 'a RiStr < S > > for NormalizationInput < 'a > { fn from (iri : & 'a RiStr < S >) -> Self { let components = RiReferenceComponents :: < S > :: from (iri . as_ref ()) ; let (scheme , authority , path , query , fragment) = components . to_major () ; let scheme = scheme . expect ("[validity] `absolute IRI must have `scheme`") ; let path = Path :: NeedsProcessing (PathToNormalize :: from_single_path (path)) ; NormalizationInput { scheme , authority , path , query , fragment , op : NormalizationOp { mode : NormalizationMode :: None , } , } } }
};
}
