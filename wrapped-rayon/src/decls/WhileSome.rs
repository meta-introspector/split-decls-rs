macro_rules! deps {
    () => {
        ParallelIterator!();
    };
}

macro_rules! WhileSome {
    () => {
        deps!();
        # [doc = " `WhileSome` is an iterator that yields the `Some` elements of an iterator,"] # [doc = " halting as soon as any `None` is produced."] # [doc = ""] # [doc = " This struct is created by the [`while_some()`] method on [`ParallelIterator`]"] # [doc = ""] # [doc = " [`while_some()`]: ParallelIterator::while_some()"] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] # [derive (Debug , Clone)] pub struct WhileSome < I > { base : I , }
    };
}

WhileSome!();