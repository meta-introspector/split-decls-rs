// Generated macro for sorted_unstable (function)
macro_rules! Depcrate_freesorted_unstable {
() => {
// Module: crate::free
// Provides: {"sorted_unstable"}
// Dependencies: {}
# [doc = " Sort all iterator elements into a new iterator in ascending order."] # [doc = " This sort is unstable (i.e., may reorder equal elements)."] # [doc = ""] # [doc = " [`IntoIterator`] enabled version of [`Itertools::sorted_unstable`]."] # [doc = ""] # [doc = " ```"] # [doc = " use itertools::assert_equal;"] # [doc = " use itertools::sorted_unstable;"] # [doc = ""] # [doc = " assert_equal(sorted_unstable(\"rust\".chars()), \"rstu\".chars());"] # [doc = " ```"] # [cfg (feature = "use_alloc")] pub fn sorted_unstable < I > (iterable : I) -> VecIntoIter < I :: Item > where I : IntoIterator , I :: Item : Ord , { iterable . into_iter () . sorted_unstable () }
};
}
