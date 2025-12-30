// Generated macro for IntoIter (struct)
macro_rules! Depcrate_optionIntoIter {
() => {
// Module: crate::option
// Provides: {"IntoIter"}
// Dependencies: {}
# [doc = " A parallel iterator over the value in [`Some`] variant of an [`Option`]."] # [doc = ""] # [doc = " The iterator yields one value if the [`Option`] is a [`Some`], otherwise none."] # [doc = ""] # [doc = " This `struct` is created by the [`into_par_iter`] function."] # [doc = ""] # [doc = " [`into_par_iter`]: IntoParallelIterator::into_par_iter()"] # [derive (Debug , Clone)] pub struct IntoIter < T > { opt : Option < T > , }
};
}
