macro_rules! deps {
    () => {
        IndexedParallelIterator!();
    };
}

macro_rules! Take {
    () => {
        deps!();
        # [doc = " `Take` is an iterator that iterates over the first `n` elements."] # [doc = " This struct is created by the [`take()`] method on [`IndexedParallelIterator`]"] # [doc = ""] # [doc = " [`take()`]: IndexedParallelIterator::take()"] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] # [derive (Debug , Clone)] pub struct Take < I > { base : I , n : usize , }
    };
}

Take!()