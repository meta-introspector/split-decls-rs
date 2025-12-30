// Generated macro for impl_141 (impl)
macro_rules! Depcrate_assert_unmovedimpl_141 {
() => {
// Module: crate::assert_unmoved
// Provides: {"impl_141"}
// Dependencies: {}
# [pinned_drop] impl < T > PinnedDrop for AssertUnmoved < T > { fn drop (self : Pin < & mut Self >) { if ! panicking () && self . this_addr != 0 { let cur_this = & * self as * const Self as usize ; assert_eq ! (self . this_addr , cur_this , "AssertUnmoved moved before drop") ; } } }
};
}
