// Generated macro for intersperse (function)
macro_rules! Depcrate_freeintersperse {
() => {
// Module: crate::free
// Provides: {"intersperse"}
// Dependencies: {}
# [doc = " Iterate `iterable` with a particular value inserted between each element."] # [doc = ""] # [doc = " [`IntoIterator`] enabled version of [`Iterator::intersperse`]."] # [doc = ""] # [doc = " ```"] # [doc = " use itertools::intersperse;"] # [doc = ""] # [doc = " itertools::assert_equal(intersperse(0..3, 8), vec![0, 8, 1, 8, 2]);"] # [doc = " ```"] pub fn intersperse < I > (iterable : I , element : I :: Item) -> Intersperse < I :: IntoIter > where I : IntoIterator , < I as IntoIterator > :: Item : Clone , { Itertools :: intersperse (iterable . into_iter () , element) }
};
}
