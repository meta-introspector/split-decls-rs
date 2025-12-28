macro_rules! deps {
    () => {
        NodeIndices!();
    };
}

macro_rules! macro_293 {
    () => {
        deps!();
        iterator_wrap ! { impl (Iterator DoubleEndedIterator ExactSizeIterator) for # [doc = " An iterator over all node indices in the graph."] # [derive (Debug , Clone)] struct NodeIndices < Ix > where { } item : Ix , iter : core :: iter :: Map < Range < usize >, fn (usize) -> Ix >, }
    };
}

macro_293!()