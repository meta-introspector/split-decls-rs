// Generated macro for Iter (struct)
macro_rules! Depcrate_optionIter {
() => {
// Module: crate::option
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " A parallel iterator over a reference to the [`Some`] variant of an [`Option`]."] # [doc = ""] # [doc = " The iterator yields one value if the [`Option`] is a [`Some`], otherwise none."] # [doc = ""] # [doc = " This `struct` is created by the [`par_iter`] function."] # [doc = ""] # [doc = " [`par_iter`]: IntoParallelRefIterator::par_iter()"] # [derive (Debug)] pub struct Iter < 'a , T > { inner : IntoIter < & 'a T > , }
};
}
