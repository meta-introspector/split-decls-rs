// Generated macro for max (function)
macro_rules! Depcrate_freemax {
() => {
// Module: crate::free
// Provides: {"max"}
// Dependencies: {}
# [doc = " Return the maximum value of the iterable."] # [doc = ""] # [doc = " [`IntoIterator`] enabled version of [`Iterator::max`]."] # [doc = ""] # [doc = " ```"] # [doc = " use itertools::max;"] # [doc = ""] # [doc = " assert_eq!(max(0..10), Some(9));"] # [doc = " ```"] pub fn max < I > (iterable : I) -> Option < I :: Item > where I : IntoIterator , I :: Item : Ord , { iterable . into_iter () . max () }
};
}
