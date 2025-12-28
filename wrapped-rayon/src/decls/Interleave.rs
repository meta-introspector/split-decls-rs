macro_rules! deps {
    () => {
        IndexedParallelIterator!();
    };
}

macro_rules! Interleave {
    () => {
        deps!();
        # [doc = " `Interleave` is an iterator that interleaves elements of iterators"] # [doc = " `i` and `j` in one continuous iterator. This struct is created by"] # [doc = " the [`interleave()`] method on [`IndexedParallelIterator`]"] # [doc = ""] # [doc = " [`interleave()`]: IndexedParallelIterator::interleave()"] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] # [derive (Debug , Clone)] pub struct Interleave < I , J > { i : I , j : J , }
    };
}

Interleave!()