// Generated macro for impl_164 (impl)
macro_rules! Depcrate_normalizeimpl_164 {
() => {
// Module: crate::normalize
// Provides: {"impl_164"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < 'a , S : Spec > From < & 'a RiString < S > > for NormalizationInput < 'a > { # [inline] fn from (iri : & 'a RiString < S >) -> Self { Self :: from (iri . as_slice ()) } }
};
}
