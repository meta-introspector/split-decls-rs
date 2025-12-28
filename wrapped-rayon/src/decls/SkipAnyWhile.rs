macro_rules! deps {
    () => {
        ParallelIterator!();
    };
}

macro_rules! SkipAnyWhile {
    () => {
        deps!();
        # [doc = " `SkipAnyWhile` is an iterator that skips over elements from anywhere in `I`"] # [doc = " until the callback returns `false`."] # [doc = " This struct is created by the [`skip_any_while()`] method on [`ParallelIterator`]"] # [doc = ""] # [doc = " [`skip_any_while()`]: ParallelIterator::skip_any_while()"] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] # [derive (Clone)] pub struct SkipAnyWhile < I , P > { base : I , predicate : P , }
    };
}

SkipAnyWhile!()