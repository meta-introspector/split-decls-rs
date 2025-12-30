// Generated macro for impl_1496 (impl)
macro_rules! Depcrate_stringimpl_1496 {
() => {
// Module: crate::string
// Provides: {"impl_1496"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "cow_str_from_iter" , since = "1.12.0")] impl < 'a , 'b > FromIterator < & 'b str > for Cow < 'a , str > { fn from_iter < I : IntoIterator < Item = & 'b str > > (it : I) -> Cow < 'a , str > { Cow :: Owned (FromIterator :: from_iter (it)) } }
};
}
