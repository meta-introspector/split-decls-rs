macro_rules! deps {
    () => {
        ParallelIterator!();
    };
}

macro_rules! Flatten {
    () => {
        deps!();
        # [doc = " `Flatten` turns each element to a parallel iterator, then flattens these iterators"] # [doc = " together. This struct is created by the [`flatten()`] method on [`ParallelIterator`]."] # [doc = ""] # [doc = " [`flatten()`]: ParallelIterator::flatten()"] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] # [derive (Debug , Clone)] pub struct Flatten < I > { base : I , }
    };
}

Flatten!();