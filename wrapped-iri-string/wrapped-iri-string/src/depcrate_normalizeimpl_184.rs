// Generated macro for impl_184 (impl)
macro_rules! Depcrate_normalizeimpl_184 {
() => {
// Module: crate::normalize
// Provides: {"impl_184"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < S : Spec > From < & Normalized < '_ , RiStr < S > > > for RiString < S > { # [inline] fn from (v : & Normalized < '_ , RiStr < S > >) -> Self { v . to_dedicated_string () } }
};
}
