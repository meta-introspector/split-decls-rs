// Generated macro for cloned (function)
macro_rules! Depcrate_freecloned {
() => {
// Module: crate::free
// Provides: {"cloned"}
// Dependencies: {}
# [doc = " Create an iterator that clones each element from `&T` to `T`."] # [doc = ""] # [doc = " [`IntoIterator`] enabled version of [`Iterator::cloned`]."] # [doc = ""] # [doc = " ```"] # [doc = " use itertools::cloned;"] # [doc = ""] # [doc = " assert_eq!(cloned(b\"abc\").next(), Some(b'a'));"] # [doc = " ```"] pub fn cloned < 'a , I , T > (iterable : I) -> iter :: Cloned < I :: IntoIter > where I : IntoIterator < Item = & 'a T > , T : Clone + 'a , { iterable . into_iter () . cloned () }
};
}
