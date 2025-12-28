macro_rules! deps {
    () => {
        ParallelIterator!();
    };
}

macro_rules! PanicFuse {
    () => {
        deps!();
        # [doc = " `PanicFuse` is an adaptor that wraps an iterator with a fuse in case"] # [doc = " of panics, to halt all threads as soon as possible."] # [doc = ""] # [doc = " This struct is created by the [`panic_fuse()`] method on [`ParallelIterator`]"] # [doc = ""] # [doc = " [`panic_fuse()`]: ParallelIterator::panic_fuse()"] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] # [derive (Debug , Clone)] pub struct PanicFuse < I > { base : I , }
    };
}

PanicFuse!()