macro_rules! deps {
    () => {
        IndexedParallelIterator!();
    };
}

macro_rules! Skip {
    () => {
        deps!();
        # [doc = " `Skip` is an iterator that skips over the first `n` elements."] # [doc = " This struct is created by the [`skip()`] method on [`IndexedParallelIterator`]"] # [doc = ""] # [doc = " [`skip()`]: IndexedParallelIterator::skip()"] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] # [derive (Debug , Clone)] pub struct Skip < I > { base : I , n : usize , }
    };
}

Skip!();