macro_rules! deps {
    () => {
        IndexedParallelIterator!();
        Take!();
        Interleave!();
    };
}

macro_rules! InterleaveShortest {
    () => {
        deps!();
        # [doc = " `InterleaveShortest` is an iterator that works similarly to"] # [doc = " `Interleave`, but this version stops returning elements once one"] # [doc = " of the iterators run out."] # [doc = ""] # [doc = " This struct is created by the [`interleave_shortest()`] method on"] # [doc = " [`IndexedParallelIterator`]."] # [doc = ""] # [doc = " [`interleave_shortest()`]: IndexedParallelIterator::interleave_shortest()"] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] # [derive (Debug , Clone)] pub struct InterleaveShortest < I , J > { interleave : Interleave < Take < I > , Take < J > > , }
    };
}

InterleaveShortest!();