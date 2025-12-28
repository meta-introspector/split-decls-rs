macro_rules! deps {
    () => {
        NodeTrait!();
        CompactDirection!();
    };
}

macro_rules! macro_891 {
    () => {
        deps!();
        iterator_wrap ! { impl (Iterator DoubleEndedIterator ExactSizeIterator) for # [derive (Debug , Clone)] struct Nodes <'a , N > where { N : 'a + NodeTrait } item : N , iter : Copied < Keys <'a , N , Vec < (N , CompactDirection) >>>, }
    };
}

macro_891!();