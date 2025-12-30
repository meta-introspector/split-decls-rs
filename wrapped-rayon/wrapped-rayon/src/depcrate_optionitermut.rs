// Generated macro for IterMut (struct)
macro_rules! Depcrate_optionIterMut {
() => {
// Module: crate::option
// Provides: {"IterMut"}
// Dependencies: {}
# [doc = " A parallel iterator over a mutable reference to the [`Some`] variant of an [`Option`]."] # [doc = ""] # [doc = " The iterator yields one value if the [`Option`] is a [`Some`], otherwise none."] # [doc = ""] # [doc = " This `struct` is created by the [`par_iter_mut`] function."] # [doc = ""] # [doc = " [`par_iter_mut`]: IntoParallelRefMutIterator::par_iter_mut()"] # [derive (Debug)] pub struct IterMut < 'a , T > { inner : IntoIter < & 'a mut T > , }
};
}
