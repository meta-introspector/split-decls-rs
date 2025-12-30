// Generated macro for impl_166 (impl)
macro_rules! Depcrate_normalizeimpl_166 {
() => {
// Module: crate::normalize
// Provides: {"impl_166"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < 'a , S : Spec > From < & 'a RiAbsoluteString < S > > for NormalizationInput < 'a > { # [inline] fn from (iri : & 'a RiAbsoluteString < S >) -> Self { Self :: from (iri . as_slice ()) } }
};
}
