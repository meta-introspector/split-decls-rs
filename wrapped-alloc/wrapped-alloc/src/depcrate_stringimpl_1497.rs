// Generated macro for impl_1497 (impl)
macro_rules! Depcrate_stringimpl_1497 {
() => {
// Module: crate::string
// Provides: {"impl_1497"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "cow_str_from_iter" , since = "1.12.0")] impl < 'a > FromIterator < String > for Cow < 'a , str > { fn from_iter < I : IntoIterator < Item = String > > (it : I) -> Cow < 'a , str > { Cow :: Owned (FromIterator :: from_iter (it)) } }
};
}
