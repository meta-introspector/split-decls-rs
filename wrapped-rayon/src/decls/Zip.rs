macro_rules! deps {
    () => {
        IndexedParallelIterator!();
    };
}

macro_rules! Zip {
    () => {
        deps!();
        # [doc = " `Zip` is an iterator that zips up `a` and `b` into a single iterator"] # [doc = " of pairs. This struct is created by the [`zip()`] method on"] # [doc = " [`IndexedParallelIterator`]"] # [doc = ""] # [doc = " [`zip()`]: IndexedParallelIterator::zip()"] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] # [derive (Debug , Clone)] pub struct Zip < A , B > { a : A , b : B , }
    };
}

Zip!();