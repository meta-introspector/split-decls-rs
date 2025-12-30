// Generated macro for min (function)
macro_rules! Depcrate_freemin {
() => {
// Module: crate::free
// Provides: {"min"}
// Dependencies: {}
# [doc = " Return the minimum value of the iterable."] # [doc = ""] # [doc = " [`IntoIterator`] enabled version of [`Iterator::min`]."] # [doc = ""] # [doc = " ```"] # [doc = " use itertools::min;"] # [doc = ""] # [doc = " assert_eq!(min(0..10), Some(0));"] # [doc = " ```"] pub fn min < I > (iterable : I) -> Option < I :: Item > where I : IntoIterator , I :: Item : Ord , { iterable . into_iter () . min () }
};
}
