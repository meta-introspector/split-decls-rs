macro_rules! deps {
    () => {
        IndexedParallelIterator!();
    };
}

macro_rules! MaxLen {
    () => {
        deps!();
        # [doc = " `MaxLen` is an iterator that imposes a maximum length on iterator splits."] # [doc = " This struct is created by the [`with_max_len()`] method on [`IndexedParallelIterator`]"] # [doc = ""] # [doc = " [`with_max_len()`]: IndexedParallelIterator::with_max_len()"] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] # [derive (Debug , Clone)] pub struct MaxLen < I > { base : I , max : usize , }
    };
}

MaxLen!()