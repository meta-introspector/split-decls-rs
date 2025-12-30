// Generated macro for IntoParallelRefMutIterator (trait)
macro_rules! Depcrate_iterIntoParallelRefMutIterator {
() => {
// Module: crate::iter
// Provides: {"IntoParallelRefMutIterator"}
// Dependencies: {}
# [doc = " `IntoParallelRefMutIterator` implements the conversion to a"] # [doc = " [`ParallelIterator`], providing mutable references to the data."] # [doc = ""] # [doc = " This is a parallel version of the `iter_mut()` method"] # [doc = " defined by various collections."] # [doc = ""] # [doc = " This trait is automatically implemented"] # [doc = " `for I where &mut I: IntoParallelIterator`. In most cases, users"] # [doc = " will want to implement [`IntoParallelIterator`] rather than implement"] # [doc = " this trait directly."] pub trait IntoParallelRefMutIterator < 'data > { # [doc = " The type of iterator that will be created."] type Iter : ParallelIterator < Item = Self :: Item > ; # [doc = " The type of item that will be produced; this is typically an"] # [doc = " `&'data mut T` reference."] type Item : Send + 'data ; # [doc = " Creates the parallel iterator from `self`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use rayon::prelude::*;"] # [doc = ""] # [doc = " let mut v = vec![0usize; 5];"] # [doc = " v.par_iter_mut().enumerate().for_each(|(i, x)| *x = i);"] # [doc = " assert_eq!(v, [0, 1, 2, 3, 4]);"] # [doc = " ```"] fn par_iter_mut (& 'data mut self) -> Self :: Iter ; }
};
}
