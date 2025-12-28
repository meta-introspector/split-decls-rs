macro_rules! deps {
    () => {
        IndexedParallelIterator!();
    };
}

macro_rules! Rev {
    () => {
        deps!();
        # [doc = " `Rev` is an iterator that produces elements in reverse order. This struct"] # [doc = " is created by the [`rev()`] method on [`IndexedParallelIterator`]"] # [doc = ""] # [doc = " [`rev()`]: IndexedParallelIterator::rev()"] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] # [derive (Debug , Clone)] pub struct Rev < I > { base : I , }
    };
}

Rev!();