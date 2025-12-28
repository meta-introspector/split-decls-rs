macro_rules! deps {
    () => {
        ParallelIterator!();
    };
}

macro_rules! Filter {
    () => {
        deps!();
        # [doc = " `Filter` takes a predicate `filter_op` and filters out elements that match."] # [doc = " This struct is created by the [`filter()`] method on [`ParallelIterator`]"] # [doc = ""] # [doc = " [`filter()`]: ParallelIterator::filter()"] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] # [derive (Clone)] pub struct Filter < I , P > { base : I , filter_op : P , }
    };
}

Filter!();