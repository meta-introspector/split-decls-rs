// Generated macro for impl_1444 (impl)
macro_rules! Depcrate_stringimpl_1444 {
() => {
// Module: crate::string
// Provides: {"impl_1444"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "box_str2" , since = "1.45.0")] impl < A : Allocator > Extend < Box < str , A > > for String { fn extend < I : IntoIterator < Item = Box < str , A > > > (& mut self , iter : I) { iter . into_iter () . for_each (move | s | self . push_str (& s)) ; } }
};
}
