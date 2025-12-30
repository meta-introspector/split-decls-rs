// Generated macro for rev (function)
macro_rules! Depcrate_freerev {
() => {
// Module: crate::free
// Provides: {"rev"}
// Dependencies: {}
# [doc = " Iterate `iterable` in reverse."] # [doc = ""] # [doc = " [`IntoIterator`] enabled version of [`Iterator::rev`]."] # [doc = ""] # [doc = " ```"] # [doc = " use itertools::rev;"] # [doc = ""] # [doc = " for elt in rev(&[1, 2, 3]) {"] # [doc = "     /* loop body */"] # [doc = "     # let _ = elt;"] # [doc = " }"] # [doc = " ```"] pub fn rev < I > (iterable : I) -> iter :: Rev < I :: IntoIter > where I : IntoIterator , I :: IntoIter : DoubleEndedIterator , { iterable . into_iter () . rev () }
};
}
