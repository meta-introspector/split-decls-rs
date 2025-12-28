macro_rules! deps {
    () => {
        ParallelIterator!();
    };
}

macro_rules! FlatMap {
    () => {
        deps!();
        # [doc = " `FlatMap` maps each element to a parallel iterator, then flattens these iterators together."] # [doc = " This struct is created by the [`flat_map()`] method on [`ParallelIterator`]"] # [doc = ""] # [doc = " [`flat_map()`]: ParallelIterator::flat_map()"] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] # [derive (Clone)] pub struct FlatMap < I , F > { base : I , map_op : F , }
    };
}

FlatMap!();