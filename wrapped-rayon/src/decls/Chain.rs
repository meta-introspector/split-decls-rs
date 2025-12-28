macro_rules! deps {
    () => {
        ParallelIterator!();
    };
}

macro_rules! Chain {
    () => {
        deps!();
        # [doc = " `Chain` is an iterator that joins `b` after `a` in one continuous iterator."] # [doc = " This struct is created by the [`chain()`] method on [`ParallelIterator`]"] # [doc = ""] # [doc = " [`chain()`]: ParallelIterator::chain()"] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] # [derive (Debug , Clone)] pub struct Chain < A , B > { a : A , b : B , }
    };
}

Chain!();