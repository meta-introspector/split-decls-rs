// Generated macro for impl_183 (impl)
macro_rules! Depcrate_normalizeimpl_183 {
() => {
// Module: crate::normalize
// Provides: {"impl_183"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < S : Spec > From < Normalized < '_ , RiStr < S > > > for RiString < S > { # [inline] fn from (v : Normalized < '_ , RiStr < S > >) -> Self { v . to_dedicated_string () } }
};
}
