// Generated macro for impl_1439 (impl)
macro_rules! Depcrate_stringimpl_1439 {
() => {
// Module: crate::string
// Provides: {"impl_1439"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "box_str2" , since = "1.45.0")] impl < A : Allocator > FromIterator < Box < str , A > > for String { fn from_iter < I : IntoIterator < Item = Box < str , A > > > (iter : I) -> String { let mut buf = String :: new () ; buf . extend (iter) ; buf } }
};
}
