macro_rules! deps {
    () => {
        IndexedParallelIterator!();
    };
}

macro_rules! is_indexed {
    () => {
        deps!();
        fn is_indexed < T : IndexedParallelIterator > (_ : T) { }
    };
}

is_indexed!()