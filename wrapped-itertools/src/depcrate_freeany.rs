// Generated macro for any (function)
macro_rules! Depcrate_freeany {
() => {
// Module: crate::free
// Provides: {"any"}
// Dependencies: {}
# [doc = " Test whether the predicate holds for any elements in the iterable."] # [doc = ""] # [doc = " [`IntoIterator`] enabled version of [`Iterator::any`]."] # [doc = ""] # [doc = " ```"] # [doc = " use itertools::any;"] # [doc = ""] # [doc = " assert!(any(&[0, -1, 2], |elt| *elt > 0));"] # [doc = " ```"] pub fn any < I , F > (iterable : I , f : F) -> bool where I : IntoIterator , F : FnMut (I :: Item) -> bool , { iterable . into_iter () . any (f) }
};
}
