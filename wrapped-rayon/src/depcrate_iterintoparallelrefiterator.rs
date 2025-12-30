// Generated macro for IntoParallelRefIterator (trait)
macro_rules! Depcrate_iterIntoParallelRefIterator {
() => {
// Module: crate::iter
// Provides: {"IntoParallelRefIterator"}
// Dependencies: {}
# [doc = " `IntoParallelRefIterator` implements the conversion to a"] # [doc = " [`ParallelIterator`], providing shared references to the data."] # [doc = ""] # [doc = " This is a parallel version of the `iter()` method"] # [doc = " defined by various collections."] # [doc = ""] # [doc = " This trait is automatically implemented"] # [doc = " `for I where &I: IntoParallelIterator`. In most cases, users"] # [doc = " will want to implement [`IntoParallelIterator`] rather than implement"] # [doc = " this trait directly."] pub trait IntoParallelRefIterator < 'data > { # [doc = " The type of the parallel iterator that will be returned."] type Iter : ParallelIterator < Item = Self :: Item > ; # [doc = " The type of item that the parallel iterator will produce."] # [doc = " This will typically be an `&'data T` reference type."] type Item : Send + 'data ; # [doc = " Converts `self` into a parallel iterator."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use rayon::prelude::*;"] # [doc = ""] # [doc = " let v: Vec<_> = (0..100).collect();"] # [doc = " assert_eq!(v.par_iter().sum::<i32>(), 100 * 99 / 2);"] # [doc = ""] # [doc = " // `v.par_iter()` is shorthand for `(&v).into_par_iter()`,"] # [doc = " // producing the exact same references."] # [doc = " assert!(v.par_iter().zip(&v)"] # [doc = "          .all(|(a, b)| std::ptr::eq(a, b)));"] # [doc = " ```"] fn par_iter (& 'data self) -> Self :: Iter ; }
};
}
