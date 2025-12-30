// Generated macro for impl_1435 (impl)
macro_rules! Depcrate_stringimpl_1435 {
() => {
// Module: crate::string
// Provides: {"impl_1435"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "rust1" , since = "1.0.0")] impl FromIterator < char > for String { fn from_iter < I : IntoIterator < Item = char > > (iter : I) -> String { let mut buf = String :: new () ; buf . extend (iter) ; buf } }
};
}
