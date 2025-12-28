macro_rules! deps {
    () => {
        IndexedParallelIterator!();
    };
}

macro_rules! Enumerate {
    () => {
        deps!();
        # [doc = " `Enumerate` is an iterator that returns the current count along with the element."] # [doc = " This struct is created by the [`enumerate()`] method on [`IndexedParallelIterator`]"] # [doc = ""] # [doc = " [`enumerate()`]: IndexedParallelIterator::enumerate()"] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] # [derive (Debug , Clone)] pub struct Enumerate < I > { base : I , }
    };
}

Enumerate!();