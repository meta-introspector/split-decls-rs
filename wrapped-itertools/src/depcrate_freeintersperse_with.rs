// Generated macro for intersperse_with (function)
macro_rules! Depcrate_freeintersperse_with {
() => {
// Module: crate::free
// Provides: {"intersperse_with"}
// Dependencies: {}
# [doc = " Iterate `iterable` with a particular value created by a function inserted"] # [doc = " between each element."] # [doc = ""] # [doc = " [`IntoIterator`] enabled version of [`Iterator::intersperse_with`]."] # [doc = ""] # [doc = " ```"] # [doc = " use itertools::intersperse_with;"] # [doc = ""] # [doc = " let mut i = 10;"] # [doc = " itertools::assert_equal(intersperse_with(0..3, || { i -= 1; i }), vec![0, 9, 1, 8, 2]);"] # [doc = " assert_eq!(i, 8);"] # [doc = " ```"] pub fn intersperse_with < I , F > (iterable : I , element : F) -> IntersperseWith < I :: IntoIter , F > where I : IntoIterator , F : FnMut () -> I :: Item , { Itertools :: intersperse_with (iterable . into_iter () , element) }
};
}
