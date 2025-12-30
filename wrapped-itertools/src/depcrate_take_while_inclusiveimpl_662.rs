// Generated macro for impl_662 (impl)
macro_rules! Depcrate_take_while_inclusiveimpl_662 {
() => {
// Module: crate::take_while_inclusive
// Provides: {"impl_662"}
// Dependencies: {}
impl < I , F > TakeWhileInclusive < I , F > where I : Iterator , F : FnMut (& I :: Item) -> bool , { # [doc = " Create a new [`TakeWhileInclusive`] from an iterator and a predicate."] pub (crate) fn new (iter : I , predicate : F) -> Self { Self { iter , predicate , done : false , } } }
};
}
