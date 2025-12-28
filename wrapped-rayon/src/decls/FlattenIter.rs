macro_rules! deps {
    () => {
        ParallelIterator!();
    };
}

macro_rules! FlattenIter {
    () => {
        deps!();
        # [doc = " `FlattenIter` turns each element to a serial iterator, then flattens these iterators"] # [doc = " together. This struct is created by the [`flatten_iter()`] method on [`ParallelIterator`]."] # [doc = ""] # [doc = " [`flatten_iter()`]: ParallelIterator::flatten_iter()"] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] # [derive (Debug , Clone)] pub struct FlattenIter < I > { base : I , }
    };
}

FlattenIter!()