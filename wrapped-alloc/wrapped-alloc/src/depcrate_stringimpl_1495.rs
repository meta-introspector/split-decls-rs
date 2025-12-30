// Generated macro for impl_1495 (impl)
macro_rules! Depcrate_stringimpl_1495 {
() => {
// Module: crate::string
// Provides: {"impl_1495"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "cow_str_from_iter" , since = "1.12.0")] impl < 'a > FromIterator < char > for Cow < 'a , str > { fn from_iter < I : IntoIterator < Item = char > > (it : I) -> Cow < 'a , str > { Cow :: Owned (FromIterator :: from_iter (it)) } }
};
}
