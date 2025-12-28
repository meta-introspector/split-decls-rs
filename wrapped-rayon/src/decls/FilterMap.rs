macro_rules! deps {
    () => {
        ParallelIterator!();
    };
}

macro_rules! FilterMap {
    () => {
        deps!();
        # [doc = " `FilterMap` creates an iterator that uses `filter_op` to both filter and map elements."] # [doc = " This struct is created by the [`filter_map()`] method on [`ParallelIterator`]."] # [doc = ""] # [doc = " [`filter_map()`]: ParallelIterator::filter_map()"] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] # [derive (Clone)] pub struct FilterMap < I , P > { base : I , filter_op : P , }
    };
}

FilterMap!();