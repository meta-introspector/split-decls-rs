// Generated macro for enumerate (function)
macro_rules! Depcrate_freeenumerate {
() => {
// Module: crate::free
// Provides: {"enumerate"}
// Dependencies: {}
# [doc = " Iterate `iterable` with a running index."] # [doc = ""] # [doc = " [`IntoIterator`] enabled version of [`Iterator::enumerate`]."] # [doc = ""] # [doc = " ```"] # [doc = " use itertools::enumerate;"] # [doc = ""] # [doc = " for (i, elt) in enumerate(&[1, 2, 3]) {"] # [doc = "     /* loop body */"] # [doc = "     # let _ = (i, elt);"] # [doc = " }"] # [doc = " ```"] pub fn enumerate < I > (iterable : I) -> iter :: Enumerate < I :: IntoIter > where I : IntoIterator , { iterable . into_iter () . enumerate () }
};
}
