// Generated macro for impl_1436 (impl)
macro_rules! Depcrate_stringimpl_1436 {
() => {
// Module: crate::string
// Provides: {"impl_1436"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "string_from_iter_by_ref" , since = "1.17.0")] impl < 'a > FromIterator < & 'a char > for String { fn from_iter < I : IntoIterator < Item = & 'a char > > (iter : I) -> String { let mut buf = String :: new () ; buf . extend (iter) ; buf } }
};
}
