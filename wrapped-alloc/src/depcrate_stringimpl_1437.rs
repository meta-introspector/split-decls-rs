// Generated macro for impl_1437 (impl)
macro_rules! Depcrate_stringimpl_1437 {
() => {
// Module: crate::string
// Provides: {"impl_1437"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "rust1" , since = "1.0.0")] impl < 'a > FromIterator < & 'a str > for String { fn from_iter < I : IntoIterator < Item = & 'a str > > (iter : I) -> String { let mut buf = String :: new () ; buf . extend (iter) ; buf } }
};
}
