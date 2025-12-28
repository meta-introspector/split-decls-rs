macro_rules! deps {
    () => {
        ParallelIterator!();
    };
}

macro_rules! TakeAnyWhile {
    () => {
        deps!();
        # [doc = " `TakeAnyWhile` is an iterator that iterates over elements from anywhere in `I`"] # [doc = " until the callback returns `false`."] # [doc = " This struct is created by the [`take_any_while()`] method on [`ParallelIterator`]"] # [doc = ""] # [doc = " [`take_any_while()`]: ParallelIterator::take_any_while()"] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] # [derive (Clone)] pub struct TakeAnyWhile < I , P > { base : I , predicate : P , }
    };
}

TakeAnyWhile!()