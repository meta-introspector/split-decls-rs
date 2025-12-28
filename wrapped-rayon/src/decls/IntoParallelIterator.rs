macro_rules! deps {
    () => {
        ParallelIterator!();
        Iter!();
    };
}

macro_rules! IntoParallelIterator {
    () => {
        deps!();
        # [doc = " `IntoParallelIterator` implements the conversion to a [`ParallelIterator`]."] # [doc = ""] # [doc = " By implementing `IntoParallelIterator` for a type, you define how it will"] # [doc = " transformed into an iterator. This is a parallel version of the standard"] # [doc = " library's [`std::iter::IntoIterator`] trait."] pub trait IntoParallelIterator { # [doc = " The parallel iterator type that will be created."] type Iter : ParallelIterator < Item = Self :: Item > ; # [doc = " The type of item that the parallel iterator will produce."] type Item : Send ; # [doc = " Converts `self` into a parallel iterator."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use rayon::prelude::*;"] # [doc = ""] # [doc = " println!(\"counting in parallel:\");"] # [doc = " (0..100).into_par_iter()"] # [doc = "     .for_each(|i| println!(\"{}\", i));"] # [doc = " ```"] # [doc = ""] # [doc = " This conversion is often implicit for arguments to methods like [`zip`]."] # [doc = ""] # [doc = " ```"] # [doc = " use rayon::prelude::*;"] # [doc = ""] # [doc = " let v: Vec<_> = (0..5).into_par_iter().zip(5..10).collect();"] # [doc = " assert_eq!(v, [(0, 5), (1, 6), (2, 7), (3, 8), (4, 9)]);"] # [doc = " ```"] # [doc = ""] # [doc = " [`zip`]: IndexedParallelIterator::zip()"] fn into_par_iter (self) -> Self :: Iter ; }
    };
}

IntoParallelIterator!();