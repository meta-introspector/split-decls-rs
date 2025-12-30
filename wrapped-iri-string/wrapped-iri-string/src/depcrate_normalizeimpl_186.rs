// Generated macro for impl_186 (impl)
macro_rules! Depcrate_normalizeimpl_186 {
() => {
// Module: crate::normalize
// Provides: {"impl_186"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < S : Spec > From < Normalized < '_ , RiAbsoluteStr < S > > > for RiAbsoluteString < S > { # [inline] fn from (v : Normalized < '_ , RiAbsoluteStr < S > >) -> Self { v . to_dedicated_string () } }
};
}
