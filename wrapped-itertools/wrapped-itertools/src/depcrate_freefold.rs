// Generated macro for fold (function)
macro_rules! Depcrate_freefold {
() => {
// Module: crate::free
// Provides: {"fold"}
// Dependencies: {}
# [doc = " Perform a fold operation over the iterable."] # [doc = ""] # [doc = " [`IntoIterator`] enabled version of [`Iterator::fold`]."] # [doc = ""] # [doc = " ```"] # [doc = " use itertools::fold;"] # [doc = ""] # [doc = " assert_eq!(fold(&[1., 2., 3.], 0., |a, &b| f32::max(a, b)), 3.);"] # [doc = " ```"] pub fn fold < I , B , F > (iterable : I , init : B , f : F) -> B where I : IntoIterator , F : FnMut (B , I :: Item) -> B , { iterable . into_iter () . fold (init , f) }
};
}
